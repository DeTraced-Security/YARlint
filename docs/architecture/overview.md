# YARlint Architecture Overview

At its most basic, YARlint is broken down into its separate functions. The CLI, file management system, linting, parsing, and input validation sections are all separated to ensure the distinction between the different sections of the YARlint pipeline. This same convention continues within each subsection of the project, where each step's substeps are made sections in their own right, taking the linearity of the project's functionality and making it modular. This allows us to more easily maintain and expand in the future with new features and options that can be added alongside eachother as pluggable modules without requiring any ground-up redesigns.

## CLI

The CLI module handles user interaction via the command line. This includes the frameworks for taking user input to run the linter and for outputting results. These represent the first and final portions of the YARlint pipeline.

## Filesystem

The filesystem module is focused on identifying YARA files. This is the first step in input validation, ensuring the linting process is only performed on files to which it pertains, whilst also serving as the interface between the host filesystem and the rest of the YARlint application.

## Validation

The validation module performs the second half of the input sanitisation. This includes further input sanitisation beyond basic filesystem checks and protects the application from malicious files.

## Parser

The parser module serves as the interface between the raw YARA file and the linting engine. It extracts information needed for linting from the YARA files for the linter to process.

## Linter

The linter is the heart of the project where the linting engine processes the parsed YARA files, comparing against the requirements set out in the cops. The results determined in the linting process are then forwarded to the CLI interface to be reported to the user.