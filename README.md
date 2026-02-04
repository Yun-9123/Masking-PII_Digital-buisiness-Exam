---
layout: article
title: masking_PII
key: home
---

#  masking_PII: Privacy-First Data Anonymization

![Rust](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Status](https://img.shields.io/badge/status-active-green)
![Platform](https://img.shields.io/badge/platform-Linux--aarch64-lightgrey)

**masking_PII** is a high-performance Command-Line Interface (CLI) tool built in Rust. It is specifically designed to automate the protection of sensitive Personal Identifiable Information (PII) for students and researchers in **Health Informatics**.

---

##  1. Why this tool? (Problem Discovery)
Researchers often handle large datasets containing sensitive student IDs or patient records. 
* **The Manual Pain:** Redacting thousands of numbers manually is slow and error-prone.
* **The Security Gap:** Online converters often violate **GDPR** by requiring cloud uploads.
* **The Solution:** Our tool provides **local, automated** masking, ensuring data privacy by design.

---

##  2. Quick Demo
If you don't have an input file ready, the tool will input a demo file for you.

```bash
# Simply run the tool without arguments
$ ./masking_PII

# Output:
# ---  masking_PII CLI Tool ---
# Notice: 'input.txt' not found. A demo file has been created for you.
# Process succeeded! Result saved to: masked_output.txt
```

---

##  3. Automation Solution
`masking_PII` leverages Rust's memory safety and the powerful `regex` crate to provide:
* **Instant Processing:** Reads input files and replaces all numerical identifiers with `*` instantly.
* **CLI Flexibility:** Easily integrated into automated pipelines via command-line arguments.
* **Reliability:** Built-in error handling for missing files, ensuring consistent performance in lab environments.

---

##  4. Marketing: How We Attract Users
* **Privacy Advocacy:** We promote the tool in medical and academic forums by highlighting its **Offline-Only** nature.
* **Developer Trust:** By hosting on GitHub and providing statically-linked binaries, we build a transparent and auditable product.
* **Seamless Integration:** We focus on users who need a lightweight tool that "just works" without complex installation.

---

##  5. Monetization: Pricing Plans

| Plan | Features | Price |
| :--- | :--- | :--- |
| **Academic** | Single file masking, Core Regex engine | **FREE** |
| **Researcher** | Batch folder processing, custom Regex patterns | **€2.99 / month** |
| **Enterprise** | GDPR compliance audit logs, API support | **Custom Pricing** |

---

##  6. Download (Binaries)
Download the latest statically linked binaries for your platform from the [Releases](https://github.com/YOUR_GITHUB_USERNAME/masking_PII/releases) page.

* 🐧 [**Linux aarch64 (musl)**](https://github.com/YOUR_GITHUB_USERNAME/masking_PII/releases) 
* 🐧 [**Linux x86_64 (musl)**](https://github.com/YOUR_GITHUB_USERNAME/masking_PII/releases)

---

*Created by a Health Informatics student for the "Digital Business" Final Exam.*
