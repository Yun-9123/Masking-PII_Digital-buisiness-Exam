![Rust](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Status](https://img.shields.io/badge/status-active-green)
![Platform](https://img.shields.io/badge/platform-Linux--aarch64-lightgrey)

**masking_PII** is a high-performance Command-Line Interface (CLI) tool built in Rust. It is specifically designed to automate the protection of sensitive Personal Identifiable Information (PII) for students and researchers in **Health Informatics**.

---

##  1. Why this tool?
Researchers often handle large datasets containing sensitive student IDs or patient records. 
* **The Manual Pain:** Redacting thousands of numbers manually is slow and error-prone.
* **The Security Gap:** Online converters often violate **GDPR** by requiring cloud uploads.
* **The Solution:** Our tool provides **local, automated** masking, ensuring data privacy by design.

---

## 2.  How to Run

## 1. Quick Demo Mode (Default)
Simply run the binary. If `input.txt` is missing, the tool will **create it for you** and process it immediately.
```bash
chmod +x masking_PII
./masking_PII
```

## 2. Custom Input Mode (For Testing Your Own Files)
To process your specific `.txt` file, pass the filename as an argument. Make sure the file is in the same folder as the binary.
```
Example: If you have a file named test.txt, run:
```bash 
./masking_PII test.txt
```

###  Supported Data Pattern
The tool utilizes a **Dual-Pattern Regex Engine** designed for strict identification:

1. **European Date Format (`DD.MM.YYYY`)**: 
   * Protects birthdates and medical records (e.g., `14.10.2025`).
2. **Continuous Long Numbers (`\b\d{5,}\b`)**: 
   * **Mobile Numbers & IDs**: Specifically targets continuous digits with 5 or more characters.
   * **Strict Format**: To ensure accuracy, phone numbers must be **connected without spaces** (e.g., `01512345678`). 
   * **Context Preservation**: Short administrative numbers (like `1` or `20`) are intentionally ignored to maintain document context.


###  Sample input.txt content:
The bundled `input.txt` is structured as a medical registration form:

```text
Anmeldung zur Untersuchung
Name: Otto Schneider
Versicherungsnummer: 0000012345
Geburtsdatum: 01.01.1990
Adresse: Schwarzwaldstrasse 9, 79098, Freiburg im Breisgau
Telefon: +491234123456
# ---------------------------------------------------------------------
Vital Signs:
Heart rate: 72bpm
Blood pressure: 120/80 mmHg
Body Temperature: 36.5 Celsius 
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
Download the latest statically linked binaries for your platform from the [Releases](https://github.com/Yun-9123/Masking-PII_Digital-buisiness-Exam/releases) page.

* 🐧 [**Linux aarch64 (musl)**](https://github.com/Yun-9123/Masking-PII_Digital-buisiness-Exam/releases/download/v0.1.2/aarch64-unknown-linux-musl.tar.gz) 
* 🐧 [**Linux x86_64 (musl)**](https://github.com/Yun-9123/Masking-PII_Digital-buisiness-Exam/releases/download/v0.1.2/x86_64-unknown-linux-musl.tar.gz)

###  Extraction Note
Please ensure you extract the **entire ZIP/TAR archive**. The executable depends on the bundled `input.txt` as a default fallback. 
-  **Included files:** `masking_PII` (Binary), `input.txt` (Required Sample).
---

*Created by a Health Informatics student for the "Digital Business" Final Exam.*
