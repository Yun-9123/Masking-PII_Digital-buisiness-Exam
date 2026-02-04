# 🛡️ masking_PII: Privacy-First Data Anonymization

**masking_PII** is a high-performance Command-Line Interface (CLI) tool built in Rust. It is designed to automate the protection of sensitive Personal Identifiable Information (PII) by replacing digits with secure masks.

---

## 🔍 1. Problem Discovery (Why this tool?)
In fields like **Health Informatics** and **Mathematics**, researchers often handle large datasets (CSV, logs, notes) containing sensitive student IDs or patient records.
* **The Manual Pain:** Manually redacting thousands of numbers is impossible and leads to human error.
* **The Security Gap:** Uploading data to online "converters" violates data protection laws (like GDPR). 
* **The Solution:** Our tool allows for **local, automated** masking, ensuring data never leaves your machine.

## 🤖 2. Automation Solution
`masking_PII` leverages Rust's memory safety and the powerful `regex` crate to provide:
* **Instant Processing:** Reads input files and replaces all numerical identifiers with `*` instantly.
* **CLI Flexibility:** Easily integrated into automated pipelines via command-line arguments.
* **Reliability:** Built-in error handling for missing files, ensuring consistent performance in lab environments.

---

## 📈 3. Marketing: How We Attract Users
* **Privacy Advocacy:** We promote the tool in medical and academic forums by highlighting its **Offline-Only** nature.
* **Developer Trust:** By hosting on GitHub and providing statically-linked binaries, we build a transparent and auditable product.
* **Seamless Integration:** We focus on users who need a lightweight tool that "just works" without complex installation.

---

## 💰 4. Monetization: Pricing Plans
| Plan | Features | Price |
| :--- | :--- | :--- |
| **Academic** | Single file masking, Core Regex engine | **FREE** |
| **Researcher** | Batch folder processing, custom Regex patterns | **€4.99 / month** |
| **Enterprise** | GDPR compliance audit logs, API support | **Custom Pricing** |

---

## 📥 5. Download (Binaries)
Please download the statically linked binary for your platform:

* 🐧 [**Linux aarch64 (musl)**](https://github.com/你的帳號/masking_PII/releases) — *Required for Exam*
* 🐧 [**Linux x86_64 (musl)**](https://github.com/你的帳號/masking_PII/releases) — *Bonus Points!*

---
*Created by a Health Informatics student for the "Build and Market your CLI Tool" Final Exam.*
