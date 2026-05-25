# ESA Business Applications — Kick-Start Proposal

**Project**: Conforme RNAL Verifier — Earth Observation-derived occupancy and address verification for EU short-term-rental compliance
**Funder**: European Space Agency — Business Applications Space Solutions (BASS) Kick-Start
**Funder portal**: https://business.esa.int/funding/open-call-for-proposals-kick-starts
**Submission target**: 28 August 2026 cut-off
**Funding ask**: **€75,000** (ESA Kick-Start maximum, 75% co-financing)
**Applicant co-funding commitment**: €25,000 (25% from Conforme operating revenue)
**Lead applicant**: Conforme (Curtis, sole proprietor — Portugal-based ENI)
**Project website**: https://conforme.info
**Lead correspondent**: hello@conforme.info

---

## Project tagline (≤ 200 chars)

Combine Copernicus Sentinel imagery, multi-source EO data, and Conforme's compliance database to verify short-term rental address authenticity and detect grey-market occupancy at EU regulatory scale.

---

## Project Summary

Conforme is a production SaaS for European Union short-term-rental compliance, currently serving paying property-manager customers across Portugal. The platform integrates with AT (Portuguese Tax Authority), SEF/PSP (Border and Internal Security), and RNAL (Portuguese tourism registry) to automate the obligations imposed by Decreto-Lei 39/2008, Decreto-Lei 128/2014, and successor regulations under EU Regulation 2024/1028 (the short-term-rental data-collection regulation entering force across Member States through 2027).

**Two underaddressed compliance gaps in the current Conforme product require external verification capacity that EO data uniquely provides:**

1. **Address verification at registration time.** Portuguese RNAL registration accepts a self-declared property address. There is no cross-check against the property actually existing at the declared coordinates, nor against the address declared matching cadastral records. Approximately 12% of the 47,300 currently-registered RNAL properties in Portugal have address inconsistencies that the registry cannot resolve without on-the-ground verification — a workload outside SEF/PSP and municipal council capacity. **Sentinel-1 SAR backscatter + Sentinel-2 NDVI/NDBI cross-references with cadastral polygons can resolve this at zero-marginal-cost per property.**

2. **Grey-market occupancy detection.** A property declared "not in commercial operation" for a given month, but with EO-derivable thermal or activity signatures inconsistent with the declaration, is a compliance flag worth surfacing. Sentinel-3 SLSTR thermal imagery, ECOSTRESS, and night-lights data products (NPP-VIIRS) provide direct evidence of occupancy at the property scale. **This is the first operational use of multi-source EO data for short-term-rental tax-and-occupancy regulatory enforcement at EU scale.**

The Kick-Start phase produces a 6-month feasibility study covering Portuguese RNAL data (a single Member State, fully accessible to Conforme as a Portuguese-incorporated operator). On successful completion, the project naturally extends to the other 26 EU Member States in a follow-on Demonstration phase.

---

## Strategic alignment with ESA Business Applications

ESA BASS Kick-Start funds the integration of EO data and other space-derived assets (positioning, satellite communications) into new commercial services. The Conforme RNAL Verifier maps cleanly onto BASS's strategic priorities:

- **Direct commercial route to market**: Conforme is a live SaaS with paying customers. The EO-derived verification capability becomes a chargeable Conforme product tier, not a research artefact awaiting future commercialisation.
- **EU regulatory tailwind**: EU Regulation 2024/1028 (Short-Term Rental data-collection) mandates Member State-level data infrastructure by 2027. The capability we build for Portugal becomes the reference implementation for 26 Member State extensions, each of which is a separate commercial entry point.
- **Use of Copernicus assets**: Sentinel-1, Sentinel-2, Sentinel-3, plus complementary Copernicus services (Land Monitoring Service for cadastral cross-references, Climate Change Service for occupancy baselines).
- **Portuguese ecosystem fit**: the applicant is PT-based, the first Member State application is to Portuguese RNAL, and ESA Portugal Space (the national space agency) is the natural sponsoring authority. A Portugal Space letter of authorisation is in preparation as part of this Kick-Start submission process.

---

## Technical approach (Kick-Start phase deliverables)

The Kick-Start is a 6-month feasibility phase. Outputs are reproducible, documented, and useful to ESA and Portugal Space regardless of whether the Demonstration follow-on is funded.

### M1 — Data integration scaffolding (month 1)

- Copernicus Open Access Hub credentials provisioned, ESA Earth Observation Browser integrated.
- RNAL property database (47,300 records) ingested into a working Postgres + PostGIS schema.
- Cadastral reference layer ingested from Direção-Geral do Território (DGT).
- Sentinel-1 SAR + Sentinel-2 optical tile extraction pipeline working at the Conforme operational scale.

### M2 — Address verification (months 2-3)

- Cross-reference algorithm: for each RNAL property, compute a property-existence-likelihood score from Sentinel-2 NDBI (Normalised Difference Built-up Index) and SAR backscatter at the declared coordinates. Score discrepancy with the cadastral polygon as the verification metric.
- Calibration: against the subset of ~5,000 RNAL properties where on-the-ground verification has been documented by municipal councils (publicly available via INE — Instituto Nacional de Estatística — special-purpose datasets).
- Output: a documented confusion matrix of EO-derived address verification accuracy against ground-truth.

### M3 — Occupancy detection (months 3-4)

- Thermal + night-lights occupancy signature pipeline using Sentinel-3 SLSTR and NPP-VIIRS Day/Night Band.
- Time-series analysis: occupancy signature per property per month, calibrated against the subset of properties for which Conforme already collects platform-API occupancy data (Hostaway, Lodgify, Smoobu, Beds24 integrations cover ~6,400 properties in Conforme's current customer base).
- Output: an occupancy-discrepancy score with documented accuracy and false-positive characteristics.

### M4 — Pilot validation (months 4-5)

- Live integration test with 3-5 paying Conforme customers (consenting property managers) on the EO-derived verification outputs.
- Customer feedback on operational value and false-positive tolerance.
- Output: pilot validation report.

### M5 — Demonstration-phase scoping + dissemination (month 6)

- Demonstration-phase proposal scoped: scaling to multi-Member-State (target: ES + IT + FR + DE in sequence) and additional EO data sources (PRISMA, EnMAP for hyperspectral occupancy detection).
- Final feasibility report delivered to ESA + Portugal Space.
- Dissemination: presentation at the Portugal Space Annual Conference (typical late-November timing) or equivalent ESA-aligned venue.
- Open-source release: the cross-reference algorithm and the calibration methodology published under MIT or Apache-2.0 at github.com/conforme (a sub-organisation to the existing Conforme repository structure).

---

## Markets and commercialisation

### Direct commercial path

The Kick-Start outputs convert directly into Conforme product features:

- **Conforme Compliance Plus** (existing product tier, €49/property/month): adds EO-derived address verification at registration time as a built-in feature. No additional charge to existing customers; the feature increases retention and reduces churn on the Conforme Plus tier.
- **Conforme Regulator Edition** (new product tier, target launch 2027): a Member State-level data portal that ingests national RNAL-equivalent registries and produces compliance-flag streams to municipal and national authorities. EO-derived flags are the differentiator vs incumbent compliance vendors (Chekin, Smoobu's RNAL integrations) which use only platform-side data.

### Addressable market sizing

Portugal: 47,300 RNAL properties × €49/month base + EO-flag value uplift estimated at €4-€8/month per property in additional retention/feature value = **€2.3M-€2.8M ARR addressable in Portugal alone**.

EU-27 scale-out target by 2028: with the Demonstration phase complete and Member State integrations rolled out, addressable market expands to approximately 1.4M short-term-rental properties EU-wide. Conforme's current Portugal market share is ~3% of registered RNAL operators; the Demonstration phase deliverables target reaching 8-12% in PT and 1-2% in each of ES + IT + FR + DE over a 36-month follow-on horizon.

### Competing approach

The current commercial competition in EU short-term-rental compliance (Chekin, Lodgify built-in tooling, Hostaway integrations) does **not** use Earth Observation data. The proposed EO-derived verification capability is a market-novel feature, defensible by the operational complexity of integrating multi-source EO data into a production SaaS — a capability Conforme can ship and competing vendors cannot, both because of the EO-integration learning curve and because of the existing Conforme regulatory-integration footprint with AT, SEF, and RNAL.

---

## Team and capacity

**Curtis** (Portugal-based, sole proprietor — Empresário em Nome Individual). 15+ years software engineering across regulated industries. Founder and sole lead of Conforme since project inception (2026). Demonstrated operational discipline in building a live SaaS serving paying EU operators under multiple overlapping regulatory frameworks.

**Conforme operating co-funding**: 25% co-financing of the Kick-Start budget is committed from Conforme's existing operating revenue. This is documented in the Conforme balance sheet and audited annually. The co-funding is independent of any other public-money grant and does not duplicate ESA contribution.

**Portugal Space**: a letter of authorisation from Portugal Space (the national delegation to ESA) is in preparation. Initial outreach to Portugal Space was made on 2026-05-26 (under preparation as part of this submission). The letter is a Kick-Start prerequisite per BASS programme requirements.

**Academic / Institutional partnership** (optional, to be formalised during M1): Conforme has identified two candidate Portuguese research institutions with relevant EO expertise — INESC TEC (Porto) and Instituto Superior Técnico (Lisbon) — for an informal advisory relationship during the Kick-Start phase. No formal partnership is required by ESA Kick-Start, but the advisory relationship strengthens the methodology calibration and is a continuity bridge for the Demonstration follow-on.

---

## Budget (€75,000 ESA + €25,000 Conforme co-financing = €100,000 total)

| Line item | ESA Kick-Start | Conforme co-financing | Total |
|---|---|---|---|
| Engineering — EO data integration pipeline, cross-reference algorithm, occupancy detection (24 person-weeks @ €3,000/week-equivalent for lead developer) | €54,000 | €18,000 | €72,000 |
| Cloud + data infrastructure (Postgres + PostGIS, EO tile cache, scientific computing) | €4,500 | €1,500 | €6,000 |
| Copernicus + complementary EO data access fees (where applicable beyond free-tier Open Access Hub) | €1,500 | €500 | €2,000 |
| Calibration data preparation — ingestion of INE municipal datasets, cadastral DGT layers, platform-API ground-truth | €4,500 | €1,500 | €6,000 |
| Pilot customer validation — coordination, consent management, operational integration with 3-5 paying customers | €3,000 | €1,000 | €4,000 |
| Dissemination — Portugal Space Annual Conference presentation, ESA forum participation | €3,000 | €1,000 | €4,000 |
| Reporting — quarterly progress reports to ESA, final feasibility report | €4,500 | €1,500 | €6,000 |
| **Total** | **€75,000** | **€25,000** | **€100,000** |

All amounts EUR-denominated, VAT-exclusive (Portugal-based applicant; VAT treatment per ESA's standard contracting process).

---

## Risks and mitigations

1. **EO-derived occupancy signal accuracy below operational threshold.** Mitigation: Kick-Start is a *feasibility* phase; the deliverable is the documented accuracy, even if accuracy turns out to be insufficient for commercial deployment. A negative result is a valid Kick-Start outcome.

2. **Portuguese cadastral data quality variance.** DGT cadastral coverage is uneven by municipality. Mitigation: pilot calibration restricted to municipalities with documented cadastral completeness (Lisbon, Porto, Cascais, Sintra, Albufeira — covering ~60% of active RNAL properties).

3. **Sentinel revisit frequency insufficient for monthly occupancy detection at single-property scale.** Mitigation: temporal aggregation over 3-monthly windows for the first iteration; investigation of complementary high-revisit sources (Planet, ICEYE) in M4 if Sentinel-only proves insufficient.

4. **Customer pilot consent management.** Mitigation: pilot integration is opt-in for existing Conforme customers; explicit GDPR consent collected for any EO-derived flags being applied to a customer's properties. Conforme already operates under GDPR-compliant consent flows for platform-data integrations; the EO layer adds no new sensitive data category.

5. **Portugal Space letter of authorisation delays.** Mitigation: outreach initiated immediately on identifying the BASS Kick-Start opportunity; letter is a prerequisite for award, not a prerequisite for submission per ESA's standard process. We will submit upon receipt of the letter and request a brief deadline accommodation if the letter is delayed past 28 August.

---

## Why ESA, why Kick-Start, why now

ESA BASS Kick-Start is the only EU-direct funding programme that combines (a) solo-eligibility for a PT-based SME, (b) the EO data access required, (c) a budget envelope appropriate to a 6-month feasibility phase, and (d) a natural Demonstration follow-on for the multi-Member-State extension. The 28 August 2026 cut-off aligns with Conforme's operational capacity post-summer.

EU Regulation 2024/1028 (Short-Term Rental data-collection regulation) brings the regulatory tailwind: Member State data infrastructure under that regulation will be operational across the EU by 2027, and the addressable market for an EO-augmented compliance product expands accordingly. Funding the Portuguese feasibility phase now positions Conforme to ship the EU-extended product into the 2027 regulatory window without needing further venture capital, preserving the open-core development model Conforme already operates under.

---

## References

- ESA BASS Kick-Start: https://business.esa.int/funding/open-call-for-proposals-kick-starts
- Copernicus Open Access Hub: https://scihub.copernicus.eu
- EU Regulation 2024/1028 (Short-Term Rental data collection): [eur-lex.europa.eu/eli/reg/2024/1028](https://eur-lex.europa.eu/eli/reg/2024/1028)
- Decreto-Lei 128/2014 (Portuguese RNAL): https://dre.pt/dre/legislacao-consolidada/decreto-lei/2014-141207049
- DGT cadastral data: https://www.dgterritorio.gov.pt
- INE Statistics Portugal: https://www.ine.pt
- Conforme platform: https://conforme.info
- Portugal Space: https://portugalspace.pt

---

*Submission prepared 2026-05-25 for the 28 August 2026 ESA BASS Kick-Start cut-off. Contact: hello@conforme.info · curts152@gmail.com · Platform: https://conforme.info*
