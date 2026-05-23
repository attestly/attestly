# Attestly outreach plan + cold-email templates

> Eight contacts to make in the next ~2 weeks. Goal: convert Andy's strategic
> directive ("focus on legitimacy, not features") into 5–8 specific actions.
> Each email below is paste-ready; replace `{{PLACEHOLDERS}}` with the live
> details before sending.

## Priority ranking

| # | Target | Asked for | Why this is high-leverage | When to send |
|---|---|---|---|---|
| 1 | **Conforme** (internal — Andy as co-signer) | 1-page letter of support naming Conforme as first pilot user | Required by both STF and EIC; no risk; just needs Andy's signature | This week |
| 2 | **Fraunhofer SIT** (Darmstadt) | Letter of academic support + advisory role | Single most credible academic signal in EU AI accountability research; their support converts STF/EIC reviewers | Mon 26 May |
| 3 | **INESC TEC** (Porto) | Letter of academic support; PT-resident research partner | PT-resident academic strengthens ANI Voucher application; geographic + linguistic match | Mon 26 May |
| 4 | **AlgorithmWatch** (Berlin) | Civil-society endorsement; potential public-policy commentary | Strongest civil-society voice in EU on AI accountability; their endorsement is the public-interest credential | Tue 27 May |
| 5 | **APDP-CNPD** (Portugal) | Brief on GDPR posture; ask for informal review | Regulator-adjacent validation; PT data-protection authority's informal feedback is gold for "regulator-runnable" claim | Wed 28 May |
| 6 | **EDRi** (Brussels) | Coalition feedback; intro to EU Parliament AI policy staff | Coalition signal + EU policy network access | Wed 28 May |
| 7 | **PwC Risk Assurance / Deloitte AI Ethics** | LinkedIn intro to AI Act lead; partnership exploration | Big-4 channel partnership = compliance-consultancy validation; named pilot if it lands | Thu 29 May |
| 8 | **CNCF CloudEvents WG** | Request to present Decision Schema v0.1 as a profile candidate | Standards-body footprint = the asymmetric long-term play | Fri 30 May |

The above list is sized for ~3 hours total of sending effort. Most replies will arrive after the 31 May submission deadline — that's fine; both STF and EIC let you supplement the application with letters received later. What matters by submission day is that the outreach is **in flight**, so each proposal can credibly say "letters of support requested from {{list}}; replies expected by {{date}}."

---

## 1. Conforme letter of support (internal — Andy signs)

**Format**: 1 page on Conforme letterhead.
**Action**: paste into a Conforme-letterhead Google Doc, Andy signs, save as PDF, attach to both STF and EIC submissions.

---

```
[Conforme letterhead]

22 May 2026

To the evaluators of the Sovereign Tech Fund application "Attestly":

I am writing in support of Curtis's Attestly project. I am co-founder of
Conforme (https://conforme.info), a Portugal-based regulatory-compliance
SaaS for EU short-term rental operators, in production with paying
customers in Portugal and Spain.

Conforme makes hundreds of automated regulatory decisions every week —
classifying providers, evaluating registration completeness, generating
compliance evidence for national tourism authorities. These decisions
fall squarely into the pattern that Attestly is designed to make
verifiable: a small operator producing legally consequential output
under regulatory oversight, where the integrity of the evidence trail
matters more than the volume.

We will be Attestly's first commercial pilot. Conforme will integrate
the Attestly SDK into our NRUA registration wizard by end of Q3 2026,
emitting signed decision events for every regulatory determination
the system makes. Conforme operators will gain a regulator-runnable
verification layer for free; Attestly gains a production deployment
and real-world performance data.

The Attestly thesis — public commitments, private payloads — is the
right architecture for the regulatory infrastructure layer Europe needs
ahead of the EU AI Act's August 2026 enforcement date. Curtis has my
unreserved professional support in this work.

Sincerely,

Andrew Nicolaou
Co-founder, Conforme
{{email}} · {{phone}}
```

---

## 2. Fraunhofer SIT — academic support

**Why this one matters**: Fraunhofer SIT is the most respected EU institute on AI accountability and secure information systems. A letter from them moves both STF and EIC evaluators. Even an interest-but-no-letter response is useful — it lets us credibly cite them as "in conversation."

**How to reach them**: their public contact is `info@sit.fraunhofer.de`. Better path is direct outreach to a named researcher. Look up the head of the "Center for Responsible AI" or the AI accountability working group on `https://www.sit.fraunhofer.de/en/`. LinkedIn is also viable.

**Suggested named target**: search "Fraunhofer SIT AI accountability" or "Fraunhofer SIT Article 12" on Google Scholar — pick the lead author of the most recent 2025/2026 paper. Direct emails to a recent publication's first author have a much higher response rate than generic addresses.

---

```
Subject: Open verification infrastructure for EU AI Act Article 12 — request for input from Fraunhofer SIT

Dear Dr. {{Surname}},

I am building Attestly, an open-source verification layer for EU AI Act
Article 12 audit evidence. I am writing to request your input on the
work, and — if it aligns with your priorities — your institute's support
for our Sovereign Tech Fund and EIC Accelerator applications.

The problem we are addressing is well-known to your research group:
Article 12 mandates automatic logging by high-risk AI systems but is
silent on log integrity. From 2 August 2026 every Annex III system in
the EU is obliged to maintain audit logs, but no off-the-shelf tooling
today produces logs whose integrity a regulator can verify
independently of the operator. The result is a regulatory framework
without a corresponding technical assurance layer.

Attestly closes the gap with a small, framework-agnostic Rust library
that records AI decisions in a tamper-evident append-only ledger,
publishes Merkle-rooted Signed Tree Heads as public commitments, and
provides a CLI verifier that any third party can run against an
exported evidence bundle. The architecture is Certificate Transparency
applied to AI accountability: public commitments, private payloads,
GDPR-compatible by construction. Apache-2.0 + CC-BY-4.0 in perpetuity.

A working PoC (Rust, 20 tests passing, full end-to-end tamper-detection
demo) is at github.com/attestly/attestly; a 27-second screencast is at
{{demo url}}. The Sovereign Tech Fund proposal is attached.

I would value:
  (a) your reaction to the technical approach — particularly the choice
      to layer on did:web identity + CloudEvents v1.0 envelope rather
      than inventing a new schema;
  (b) if SIT's roadmap is compatible, a brief letter of support naming
      Fraunhofer SIT as an interested academic collaborator. Even a
      formal "we have read this and consider the direction credible"
      would carry weight with the funders.

I expect to submit the STF and EIC applications on 31 May. A reply by
30 May would be ideal but a later one still has value — both funders
accept post-submission letters as supplements.

Happy to schedule a 30-minute call if you would prefer to discuss.

With thanks,

Curtis
{{phone}} · curts152@gmail.com
github.com/attestly/attestly
```

---

## 3. INESC TEC — Portuguese academic anchor

**Why**: Portugal-resident academic affiliation strengthens the ANI Voucher Deep Tech case (PT national funding) and the EIC's "EU strategic interest" framing.

**How to reach**: `https://www.inesctec.pt/en/`. Likely contact group: HASLab (the High-Assurance Software Lab in Braga) or CRACS (Center for Research in Advanced Computing Systems in Porto). HASLab specifically does verified systems + cryptography — a clean fit.

**Suggested approach**: identify a HASLab researcher publishing on verifiable computing, applied cryptography, or transparency systems. Email them in English (most HASLab researchers publish in English and respond in English; Portuguese is fine if you prefer).

---

```
Subject: Open verification infrastructure for EU AI Act Article 12 — collaboration interest from a Portugal-based developer

Caro/a Dr/a. {{Surname}},

Permita-me apresentar-me em inglês para clareza técnica.

I am a Portugal-based developer (Lisbon) building Attestly, an
open-source verification layer for EU AI Act Article 12 audit
evidence. I have a working PoC in Rust (20 tests passing, full
tamper-detection demo) and am in the final week of preparing
applications to the Sovereign Tech Fund and the EIC Accelerator.

The technical core is at HASLab's intersection: an append-only signed
event ledger, Merkle-rooted commitments published to a public log,
Ed25519 signing throughout, did:web identity for the operator
organisation, and a separate verifier crate that operates with no
database dependency — what a regulator would run locally to
independently check an exported evidence bundle. The architecture is
deliberately Certificate Transparency applied to AI decisions: public
commitments, private payloads, GDPR-compatible by construction.

A 27-second screencast of the tamper-detection demo is at {{demo
url}}; the repository is github.com/attestly/attestly; the Sovereign
Tech Fund proposal is attached.

I would be grateful for INESC TEC's interest in two ways:

  (a) An academic partner / advisor relationship — the project would
      gain enormously from having a HASLab researcher providing
      informal technical review of the wire-format specification and
      the Merkle/checkpoint design. No formal funding obligation
      either way.

  (b) If feasible by 30 May, a brief letter of support naming INESC
      TEC as a potential research collaborator. This strengthens both
      the Sovereign Tech Fund application and the ANI Voucher Deep
      Tech follow-on at the national level.

Posso explicar tudo em português numa chamada se for mais conveniente.

Cordialmente,

Curtis
{{phone}} · curts152@gmail.com
```

---

## 4. AlgorithmWatch — civil-society voice

**Why**: AlgorithmWatch is the highest-impact civil-society voice in EU on AI accountability. They have the policy networks; their endorsement is the public-interest credential that grant evaluators recognise immediately.

**How to reach**: `contact@algorithmwatch.org`. They publish names of policy leads on their site (algorithmwatch.org/en/team/) — direct emails to the relevant lead (typically the policy team for AI Act work) get faster replies than the generic address.

**Caveat**: AlgorithmWatch is a Berlin-based NGO with limited bandwidth. Lower probability of a letter than the academic ones, but even a "we are aware of this work" public mention is useful.

---

```
Subject: Open verification infrastructure for EU AI Act Article 12 — request for civil-society input

Dear {{Name}} / AlgorithmWatch team,

I have followed AlgorithmWatch's work on EU AI Act enforcement closely
and am writing to share a project that I believe sits in your area of
concern.

From 2 August 2026, Article 12 of the AI Act will require every
high-risk AI system in the EU to maintain automatic logs of operation.
The Act is silent on whether those logs can be trusted. Today an
operator can edit their own audit trail before a regulator inspects it
— and no off-the-shelf open-source tool exists that lets a regulator
or an affected citizen detect such editing.

This is the gap Attestly addresses. It is a small open-source library
(Apache-2.0) that records every AI decision in a tamper-evident
ledger, publishes cryptographic commitments — never the decisions
themselves — to a public log, and provides a standalone command-line
verifier that any third party can run with no operator cooperation.
The architecture follows Certificate Transparency: public commitments,
private payloads, GDPR-compatible by construction. Affected citizens
can verify a contested decision themselves.

A 27-second screencast showing the tamper-detection working
end-to-end is at {{demo url}}; the code is at
github.com/attestly/attestly; the Sovereign Tech Fund proposal is
attached.

I would value AlgorithmWatch's reaction in any of three forms:

  (a) Informal feedback on whether the approach addresses a problem
      you consider important, and whether the framing — public
      commitments, private payloads — is one you would publicly
      endorse.

  (b) A short statement of support, formal or informal, that I could
      attach to the Sovereign Tech Fund and EIC Accelerator
      applications. Even a paragraph that we could quote would be
      enormously helpful.

  (c) An introduction to policy contacts at EU Parliament or
      ENISA-adjacent groups working on AI Act implementation.

I expect to submit on 31 May; replies after that are equally welcome
because both funders accept post-submission supplements.

Happy to schedule a call if useful.

With thanks for the work AlgorithmWatch does,

Curtis
{{phone}} · curts152@gmail.com
github.com/attestly/attestly
```

---

## 5. APDP-CNPD (Portuguese Data Protection Commissioner)

**Why**: PT data-protection authority's reaction to the GDPR posture (commitments public, payloads private) is the strongest possible signal we can attach to the "regulator-runnable" claim. Even an informal acknowledgement that the approach is GDPR-compatible is gold.

**How to reach**: `geral@cnpd.pt`. Public contact. CNPD has a published process for "consultas prévias" but a direct technical query is also accepted.

**Tone**: formal, respectful, Portuguese government register. This email is in Portuguese for cultural appropriateness; the technical document attached is in English.

---

```
Assunto: Atestly — infraestrutura aberta de verificação para o artigo 12 do Regulamento (UE) 2024/1689 (Lei IA) — pedido de comentários sobre conformidade RGPD

Exmos. Senhores,

Sou um programador português residente em Lisboa, a desenvolver um
projeto de código aberto — Attestly — desenhado para responder à
obrigação de registo automático imposta pelo artigo 12 do Regulamento
(UE) 2024/1689 (Lei da IA), aplicável a sistemas de IA de risco
elevado a partir de 2 de agosto de 2026.

O projeto está em fase pré-submissão para o Sovereign Tech Fund e o
EIC Accelerator. Antes de submeter, agradeceria muito a possibilidade
de obter uma reação informal da CNPD sobre o desenho de proteção de
dados que adotámos.

Resumo da arquitetura:

  (1) Os pagamentos das decisões da IA permanecem na base de dados
      do operador. Nada é publicado em claro.

  (2) Apenas resumos criptográficos (hashes SHA-256) e compromissos
      assinados sobre uma raiz de árvore de Merkle são publicados num
      registo público. Estes resumos não permitem reconstruir as
      decisões originais sem cooperação do operador.

  (3) O regulador (ou cidadão afetado) pode verificar a integridade
      de uma decisão específica recebendo um pacote exportado pelo
      operador, sem qualquer acesso à base de dados deste.

O padrão arquitetónico é o do Certificate Transparency aplicado a
decisões de IA: compromissos públicos, conteúdos privados.

A nossa interpretação é que este desenho é compatível com o RGPD
porque os artefactos publicamente acessíveis não constituem dados
pessoais na aceção do artigo 4.º — são digests criptográficos sem
capacidade de reidentificação.

Anexo a documentação técnica em inglês e um curto vídeo
demonstrativo. Agradecíamos qualquer indicação informal — mesmo que
preliminar — sobre se este desenho levanta preocupações no plano da
proteção de dados que devamos considerar antes da submissão de 31 de
maio.

Estamos, naturalmente, disponíveis para qualquer esclarecimento ou
reunião que considerem útil.

Com os melhores cumprimentos,

Curtis
{{phone}} · curts152@gmail.com
github.com/attestly/attestly
```

---

## 6. EDRi (European Digital Rights) — Brussels coalition

**Why**: EDRi is a Brussels-based coalition of digital-rights organisations across Europe. Membership gives access to EU policy networks: Parliament staff, Commission consultations, civil-society coalition statements. Slower-moving than AlgorithmWatch but more politically connected.

**How to reach**: `brussels@edri.org`. Specific policy leads listed at edri.org/about/team/.

---

```
Subject: Open verification infrastructure for EU AI Act Article 12 — civil-society alignment

Dear EDRi team,

I am writing from Lisbon to introduce Attestly, an open-source
verification layer for EU AI Act Article 12 evidence, and to explore
whether it might align with EDRi's coalition work on AI accountability.

The Article 12 logging mandate becomes enforceable on 2 August 2026,
but the Act is silent on the integrity of those logs. Today an
operator of a high-risk AI system can edit its own audit trail before
a regulator inspects it. This gap is well-documented in the policy
literature (cf. AlgorithmWatch's recent commentary) but currently has
no open-source remedy.

Attestly is a small, framework-agnostic library — Apache-2.0,
specification CC-BY-4.0 — that records every AI decision in a
tamper-evident ledger, publishes cryptographic commitments to a
public log, and provides a standalone verifier any third party can
run with no operator cooperation. The architecture follows
Certificate Transparency, applied to AI decisions. Public
commitments; private payloads; GDPR-compatible by construction.

A 27-second tamper-detection screencast is at {{demo url}}; the
code is at github.com/attestly/attestly; the Sovereign Tech Fund
proposal is attached.

I would value EDRi's reaction in two ways:

  (a) Whether the project aligns with EDRi's policy priorities on AI
      Act implementation and on data subject rights — and whether
      EDRi would consider issuing a brief statement of support or
      including Attestly in upcoming coalition work.

  (b) An introduction to relevant EU Parliament staff or Commission
      DG CNECT contacts working on AI Act implementation guidelines.

Submitting STF and EIC by 31 May; later replies welcome.

With thanks for EDRi's ongoing work,

Curtis
{{phone}} · curts152@gmail.com
```

---

## 7. Big-4 EU advisory — PwC Risk Assurance or Deloitte AI Ethics

**Why**: Big-4 advisory practices sell EU AI Act compliance to enterprise customers. A relationship — even informal — gives Attestly two things: (i) channel access to enterprise pilots that Curtis can't reach directly, and (ii) market validation that grant evaluators read.

**How to reach**: LinkedIn is the right channel. Search for "AI Act compliance" + the firm name in your network. Both firms publish their AI Ethics / Risk leads' names on their EU-region sites:

- **PwC EU AI Act practice**: https://www.pwc.com/gx/en/services/ai/responsible-ai.html — look for the EU-region lead
- **Deloitte AI Institute**: https://www2.deloitte.com/global/en/pages/about-deloitte/articles/ai-institute.html — find the EMEA AI Ethics lead

**Caveat**: Big-4 cold contact has low reply rate. Better path is a warm intro via Andy or any Conforme customer network. Below is a template for whichever path lands.

---

```
Subject: Open verification infrastructure for EU AI Act Article 12 — exploration of channel partnership

Dear {{Name}},

I am the founder of Attestly, an open-source verification layer for
EU AI Act Article 12 audit evidence. We are in the final week of
applications to the Sovereign Tech Fund and the EIC Accelerator, and
are looking to begin pilot conversations with enterprise compliance
consultancies in the EU.

The thesis is short: from 2 August 2026 every operator of an Annex III
high-risk AI system is legally obliged to maintain integrity-verifiable
audit logs. The current generation of AI governance tooling
(Langfuse, AgentOps, Microsoft Agent Governance Toolkit) provides
observability but not regulator-runnable independent verification. We
have built the open primitive that closes that gap — Rust core +
language SDKs + standalone WASM verifier — and are operating
deliberately as commons infrastructure rather than a closed product.

I would value a 30-minute conversation about:

  (1) Whether {{PwC / Deloitte}}'s EU AI Act practice sees the
      audit-log integrity problem in client engagements today, and
      whether Attestly's framework-agnostic approach maps to what
      enterprise clients are actually being told they need.

  (2) Whether there is appetite — even informal — for a channel
      partnership where Attestly handles the technical layer and
      {{PwC / Deloitte}} handles the audit-pack delivery and
      regulatory advisory layer for end clients.

A 27-second demo of the tamper-detection working end-to-end is at
{{demo url}}; the repository is github.com/attestly/attestly.

Happy to fit your calendar in the next two weeks.

With thanks,

Curtis
{{phone}} · curts152@gmail.com
```

---

## 8. CNCF CloudEvents WG — standards-body footprint

**Why**: this is the long-game move. If the Attestly Decision Schema gets accepted as a CloudEvents profile by the CNCF working group, every CloudEvents-compatible system in the world becomes Attestly-interoperable. That is the asymmetric event Andy referred to. Even being a participant — not yet a contributor — strengthens the proposal's "standards engagement" claim.

**How to reach**: the CloudEvents WG runs on GitHub at `github.com/cloudevents/spec` and has a weekly Zoom meeting (Wednesdays, schedule at the repo). The lowest-friction first contact is to:

  1. Open a GitHub Discussion at cloudevents/spec proposing the AI-decision-event profile concept
  2. Send the email below to the WG chairs (visible on the repository)

---

```
Subject: Proposal: AI-decision-event profile for CloudEvents v1.0 — invitation for working-group review

Dear CloudEvents Working Group chairs,

I am preparing a proposed CloudEvents profile for AI-decision events
in the context of EU AI Act Article 12 compliance, and would like to
introduce it to the working group for early-stage review.

Background: from 2 August 2026 the EU AI Act mandates automatic
logging by all high-risk AI systems. There is an open architectural
gap — these logs are not currently tamper-evident in a
regulator-verifiable way. I am building Attestly
(github.com/attestly/attestly), an Apache-2.0 verification layer
based on CloudEvents v1.0 as the envelope format, Ed25519-signed
events, Merkle-rooted Signed Tree Heads published to a public log,
and a standalone WASM verifier for regulator use.

The Decision Schema I have drafted (v0.1) is a thin extension on
CloudEvents v1.0:

  attestlyschemaversion       — "0.1"
  attestlydecisioncategory    — AI Act Annex III category
                                ("credit_score", "employment",
                                "biometric", "health_diagnostic", etc.)
  attestlymodelid             — operator-defined model identifier
  attestlysubjectref          — pseudonymous SHA-256 subject ref

These attributes carry no PII; the `data` field remains
operator-defined.

I would value the working group's early reaction in two stages:

  (1) Should this be advanced as a CloudEvents profile candidate?

  (2) Is there an existing WG member who has worked on
      regulatory-event profiles (the medical / financial data spaces)
      whose precedent we should align with?

I am happy to present at an upcoming Wednesday WG call if there is
interest. The Attestly project is otherwise in the final week of
applications to the Sovereign Tech Fund and the EIC Accelerator;
working-group acknowledgement of the proposal is not a dependency for
those submissions, but a public reception of the proposal in the
WG's GitHub Discussions area would strengthen the standards-engagement
section of both applications.

With thanks,

Curtis
{{phone}} · curts152@gmail.com
github.com/attestly/attestly
```

---

## After-send tracking

Keep a simple log (e.g. `outreach-log.md` in this directory) of each
email sent: target, date sent, reply received y/n, decision made.

If response rate by 30 May is ≥ 3 of 8: cite the in-flight outreach in
both proposals' "letters of support" section verbatim, with
acknowledgement that replies are expected post-submission.

If response rate is < 3 of 8: still cite the outreach in flight, but
don't make grand claims; let the work speak for itself.

The submission must not be delayed for outreach replies. STF accepts
post-submission supplements at any time, and EIC's monthly cut-off
means a missed deadline is a 30-day delay — far worse than a leaner
"letters in flight" claim.
