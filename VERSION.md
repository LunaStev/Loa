# Version Management Rules
This document outlines the version management rules for the Wave Compiler project. The versioning system is structured to track the progression of features and to provide clarity on the development stage at any given point. Each version number corresponds to a specific stage of development, showcasing the cumulative progress of the project.

---

## Development Stages
Versions are categorized according to the following development stages, which represent the functionality and stability level of the project:

* **pre-alpha**: The very early stage of development, where core features are not yet implemented, and only basic tasks such as token parsing and AST output are handled.
* **pre-beta**: The minimum viable features are implemented, and some basic execution is possible. This stage mainly focuses on internal testing and feature additions.
* **alpha**: The initial alpha stage where key features are implemented, and basic functionality is working, though there may still be many bugs.
* **beta**: The core functionality is operational, and the project is now testable. External users can begin to test the software.
* **rc (Release Candidate)**: A release candidate version, used for final stability checks and any last-minute fixes before the official release.
* **stable/release**: The official release version, where all features are stable and the software is ready for general use.

---

## Version Numbering Rules
Version numbers are incremented based on feature changes and stage transitions. The versioning follows this pattern:

* Version numbers reset when transitioning between stages. For example, moving from `0.0.7-pre-alpha` to `0.0.1-pre-beta` involves resetting the version number for the new stage.
* Updates to the project (such as feature additions or bug fixes) result in the version number increasing, but within the same stage. For example, a feature update in the pre-beta stage will result in a version change from `0.0.4-pre-beta` to `0.0.5-pre-bet`a.

---

## Example Version Flow
Here’s an example of how versions might increment, along with the significant changes introduced at each stage:

* **0.0.7-pre-alpha**: Basic token parsing and AST output functionality.
* **0.0.8-pre-alpha**: Improvements to token handling and AST output.
* **0.0.1-pre-beta**: LLVM output functionality added.
* **0.0.2-pre-beta**: Added support for print and println functions.
* **0.0.3-alpha**: Improvements to syntax handling and parser.
* **0.1.0-beta**: Key features implemented, external testing enabled.
* **0.1.1-beta**: Bug fixes and optimizations.
* **0.2.0-rc**: Final stability checks and minor fixes.
* **0.2.0**: Stable release version.

---

## Version Management Summary
* **Each version corresponds to a specific development stage.**
* **Version numbers increase as new features or changes are made**, but reset when transitioning to a new development stage (e.g., from `0.0.7-pre-alpha` to `0.0.1-pre-beta`).
* **Major feature developments** and changes are reflected in the version increments for each stage.