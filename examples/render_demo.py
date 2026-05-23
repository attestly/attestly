"""
Render an animated GIF of the Attestly demo.

Drives the real `attestly` binary via subprocess, captures actual output,
and composites terminal-style frames into a GIF. Does NOT require pty/fcntl
(so it works on Windows where asciinema doesn't).

Output: demo-out/attestly-demo.gif
"""

from __future__ import annotations

import os
import re
import subprocess
import sys
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

from PIL import Image, ImageDraw, ImageFont

# ---------- look + feel ----------
WIDTH, HEIGHT = 1200, 720
PADDING_X, PADDING_Y = 18, 14
FONT_PATH = r"C:\Windows\Fonts\consola.ttf"
FONT_BOLD_PATH = r"C:\Windows\Fonts\consolab.ttf"
FONT_SIZE = 16
LINE_HEIGHT = 22

BG = (15, 16, 22)
FG = (215, 215, 220)
PROMPT = (110, 195, 130)
CMD = (240, 235, 145)
HEADING = (130, 195, 235)
OK_COLOR = (130, 220, 140)
FAIL_COLOR = (240, 130, 130)
WARN_COLOR = (240, 200, 130)
DIM = (130, 130, 145)

ANSI_RE = re.compile(r"\x1b\[[0-9;]*m")


@dataclass
class Line:
    text: str
    color: tuple[int, int, int] = FG
    bold: bool = False


@dataclass
class Terminal:
    lines: list[Line] = field(default_factory=list)
    max_rows: int = 28

    def add(self, text: str, color=FG, bold=False) -> None:
        for sub in text.split("\n"):
            self.lines.append(Line(sub, color, bold))

    def snapshot(self) -> list[Line]:
        return list(self.lines[-self.max_rows :])


def load_fonts() -> tuple[ImageFont.FreeTypeFont, ImageFont.FreeTypeFont]:
    return (
        ImageFont.truetype(FONT_PATH, FONT_SIZE),
        ImageFont.truetype(FONT_BOLD_PATH, FONT_SIZE),
    )


def render_frame(term: Terminal) -> Image.Image:
    img = Image.new("RGB", (WIDTH, HEIGHT), BG)
    draw = ImageDraw.Draw(img)
    font, font_bold = load_fonts()

    # title bar
    draw.rectangle((0, 0, WIDTH, 32), fill=(28, 30, 38))
    draw.ellipse((10, 10, 22, 22), fill=(232, 95, 95))
    draw.ellipse((28, 10, 40, 22), fill=(232, 182, 88))
    draw.ellipse((46, 10, 58, 22), fill=(120, 196, 120))
    draw.text((WIDTH // 2 - 90, 8), "attestly demo", fill=DIM, font=font)

    y = 40 + PADDING_Y
    for line in term.snapshot():
        f = font_bold if line.bold else font
        draw.text((PADDING_X, y), line.text, fill=line.color, font=f)
        y += LINE_HEIGHT
    return img


def strip_ansi(s: str) -> str:
    return ANSI_RE.sub("", s)


def colorize(line: str) -> tuple[str, tuple[int, int, int], bool]:
    stripped = strip_ansi(line)
    s = stripped.rstrip()
    if "[FAIL]" in s or "TAMPERED" in s or s.startswith("ERROR"):
        return s, FAIL_COLOR, True
    if "[OK]" in s:
        return s, OK_COLOR, True
    if s.startswith("⚠") or s.startswith("Warning") or "WARN" in s:
        return s, WARN_COLOR, False
    if s.startswith("=== ") and s.endswith(" ==="):
        return s, HEADING, True
    if s.startswith("✓"):
        return s, OK_COLOR, False
    return s, FG, False


@dataclass
class Step:
    cmd_display: str  # what to show after the prompt
    cmd_args: Optional[list[str]] = None  # actual command-line args (or None if replay-only)
    pre_pause_ms: int = 400  # pause after typing, before run
    post_pause_ms: int = 900  # pause after output completes
    skip_output: bool = False  # don't show stdout (e.g. silent appends)
    expect_nonzero: bool = False  # this step is supposed to exit non-zero
    replay_stdout: Optional[str] = None  # use pre-captured output instead of running


def make_frames(steps: list[Step], attestly: Path, workdir: Path) -> list[tuple[Image.Image, int]]:
    """Returns list of (frame, duration_ms)."""
    term = Terminal(max_rows=28)
    frames: list[tuple[Image.Image, int]] = []

    def snapshot(duration: int) -> None:
        frames.append((render_frame(term), duration))

    # opening card
    term.add("Attestly — EU AI Act Article 12 evidence integrity", HEADING, True)
    term.add("", FG)
    term.add("Drop-in tamper-evident audit log + public verifier", DIM)
    term.add("for any high-risk AI system.", DIM)
    term.add("", FG)
    term.add("github.com/attestly", DIM)
    snapshot(2400)

    # clear and start running
    term.lines = []
    snapshot(300)

    for step in steps:
        # show typing the command (animate by appending the prompt+command)
        term.add(f"$ {step.cmd_display}", CMD)
        snapshot(step.pre_pause_ms)

        # source the output: either replay a pre-captured string, or run the command
        stdout_text: str = ""
        stderr_text: str = ""
        returncode: int = 0
        if step.replay_stdout is not None:
            stdout_text = step.replay_stdout
        elif step.cmd_args is not None:
            result = subprocess.run(
                [str(attestly)] + step.cmd_args,
                cwd=workdir,
                capture_output=True,
                text=True,
                encoding="utf-8",
                errors="replace",
            )
            stdout_text = result.stdout or ""
            stderr_text = result.stderr or ""
            returncode = result.returncode

        if not step.skip_output:
            output_lines = stdout_text.splitlines() + stderr_text.splitlines()
            # stream output, snapshotting after every 1-2 lines for visible pacing
            buffer = 0
            for ol in output_lines:
                txt, color, bold = colorize(ol)
                term.add(txt, color, bold)
                buffer += 1
                if buffer >= 2:
                    snapshot(150)
                    buffer = 0
            if buffer > 0:
                snapshot(150)

        # pause to let viewer absorb
        snapshot(step.post_pause_ms)

        # sanity-check exit code (skip when replaying or no command)
        if step.cmd_args is not None and step.replay_stdout is None:
            if step.expect_nonzero and returncode == 0:
                print(f"WARN: expected nonzero exit but got 0 for: {step.cmd_args}", file=sys.stderr)
            if not step.expect_nonzero and returncode != 0:
                print(f"WARN: expected zero exit but got {returncode} for: {step.cmd_args}", file=sys.stderr)
                print(f"      stderr: {stderr_text}", file=sys.stderr)

    # closing card
    term.lines = []
    term.add("", FG)
    term.add("                Tamper detection works end-to-end.", OK_COLOR, True)
    term.add("", FG)
    term.add("                Public commitments. Private payloads.", FG)
    term.add("                Independently verifiable. GDPR-compatible.", FG)
    term.add("", FG)
    term.add("                github.com/attestly", HEADING, True)
    snapshot(3200)

    return frames


def main() -> int:
    project_root = Path(__file__).resolve().parent.parent
    attestly = project_root / "target" / "release" / "attestly.exe"
    if not attestly.exists():
        print(f"ERROR: build the binary first: cargo build --release ({attestly})", file=sys.stderr)
        return 1

    workdir = project_root / "demo-out"
    if workdir.exists():
        import shutil

        shutil.rmtree(workdir)
    workdir.mkdir(parents=True)

    # 1. init
    init_args = [
        "init",
        "--db", "attestly.db",
        "--system-did", "did:web:bank.example/ai-system/credit-scorer",
        "--org-did", "did:web:bank.example",
        "--key-id", "ops-2026-05",
        "--keys-dir", "keys",
    ]

    # 2. one "representative" append for the recording (we'll do the
    # remaining 49 silently so the GIF stays short). seq=24 will be 'denied'.
    def append_args(i: int):
        decision = "denied" if i % 3 == 0 else "approved"
        score = ((i * 37) % 100) / 100.0
        data = f'{{"decision":"{decision}","score":{score:.2f},"applicant":"applicant-{i}"}}'
        return [
            "append",
            "--db", "attestly.db",
            "--keys-dir", "keys",
            "--data", data,
        ]

    # Pre-flight: actually run init + 50 appends now, so visible steps can
    # either run for real or replay captured output without re-running.
    print("Pre-flight: running init + 50 appends silently to set up state...", file=sys.stderr)
    init_result = subprocess.run(
        [str(attestly)] + init_args,
        cwd=workdir,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=True,
    )
    for i in range(1, 51):
        subprocess.run(
            [str(attestly)] + append_args(i),
            cwd=workdir,
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            check=True,
        )
    print(f"Pre-flight complete. init stdout: {len(init_result.stdout)} bytes", file=sys.stderr)

    # Visible steps. Init's output is replayed from the pre-flight capture
    # so we don't have to re-init (which would generate fresh keys and
    # invalidate the 50 already-signed events).
    visible_steps: list[Step] = [
        Step(
            cmd_display=(
                "attestly init --db attestly.db \\\n  "
                "--system-did did:web:bank.example/ai-system/credit-scorer \\\n  "
                "--org-did did:web:bank.example --keys-dir keys"
            ),
            replay_stdout=init_result.stdout,
            post_pause_ms=1400,
        ),
        Step(
            cmd_display="for i in 1..50; do attestly append --data ...; done",
            replay_stdout="    50 synthetic credit decisions appended (signed, hash-chained)",
            post_pause_ms=1200,
        ),
    ]

    visible_steps += [
        Step(
            cmd_display="attestly publish-checkpoint --out checkpoint.json",
            cmd_args=[
                "publish-checkpoint",
                "--db", "attestly.db",
                "--keys-dir", "keys",
                "--out", "checkpoint.json",
            ],
            post_pause_ms=1600,
        ),
        Step(
            cmd_display="attestly export --decision-id 24 --for-regulator",
            cmd_args=[
                "export",
                "--db", "attestly.db",
                "--keys-dir", "keys",
                "--decision-id", "24",
                "--checkpoint", "checkpoint.json",
                "--out", "regulator-24.zip",
            ],
            post_pause_ms=1600,
        ),
        Step(
            cmd_display="attestly verify --bundle regulator-24.zip",
            cmd_args=["verify", "--bundle", "regulator-24.zip"],
            post_pause_ms=2200,
        ),
        Step(
            cmd_display="# --- operator tampers after the fact ---",
            cmd_args=["--version"],  # harmless no-op for the typing animation
            post_pause_ms=600,
            skip_output=True,
        ),
        Step(
            cmd_display='attestly demo-tamper --seq 24 --new-decision "approved"',
            cmd_args=[
                "demo-tamper",
                "--db", "attestly.db",
                "--seq", "24",
                "--new-decision", "approved",
            ],
            post_pause_ms=1800,
        ),
        Step(
            cmd_display="attestly export --decision-id 24 --for-regulator  # re-export",
            cmd_args=[
                "export",
                "--db", "attestly.db",
                "--keys-dir", "keys",
                "--decision-id", "24",
                "--checkpoint", "checkpoint.json",
                "--out", "regulator-24-tampered.zip",
            ],
            post_pause_ms=1400,
        ),
        Step(
            cmd_display="attestly verify --bundle regulator-24-tampered.zip",
            cmd_args=["verify", "--bundle", "regulator-24-tampered.zip"],
            post_pause_ms=3000,
            expect_nonzero=True,
        ),
    ]

    print(f"Rendering {len(visible_steps)} visible steps...", file=sys.stderr)
    frames = make_frames(visible_steps, attestly, workdir)

    out_gif = workdir / "attestly-demo.gif"
    print(f"Compositing {len(frames)} frames to {out_gif}...", file=sys.stderr)
    images = [f for f, _ in frames]
    durations = [d for _, d in frames]
    images[0].save(
        out_gif,
        save_all=True,
        append_images=images[1:],
        duration=durations,
        loop=0,
        optimize=False,
        disposal=2,
    )
    total_ms = sum(durations)
    size_mb = out_gif.stat().st_size / (1024 * 1024)
    print(f"OK: {out_gif} ({total_ms / 1000:.1f}s, {size_mb:.1f} MB, {len(frames)} frames)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
