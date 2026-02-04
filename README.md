# 🛡️ masking_PII: Privacy-First Data Anonymization

**masking_PII** is a high-performance Command-Line Interface (CLI) tool built in Rust. It is designed to automate the protection of sensitive Personal Identifiable Information (PII) by replacing digits with secure masks.

---

##  1. Why this tool?
In field in **Health Informatics**, researchers often handle large datasets (CSV, logs, notes) containing sensitive student IDs or patient records.
* **The Manual Pain:** Manually redacting thousands of numbers is impossible and leads to human error.
* **The Security Gap:** Uploading data to online "converters" violates data protection laws (like GDPR). 
* **The Solution:** Our tool allows for **local, automated** masking, ensuring data never leaves your machine.

---

##  2. Quick Demo
Run the tool with a single command:
```bash
# How to use
$ ./masking_PII input.txt

# What happens
File_input: input.txt
Process succeeded, see masked_output.txt
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
