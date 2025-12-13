//! Utilities Module for cmdcreate
//!
//! This module provides various utility functions and constants
//! used throughout cmdcreate. These utilities simplify common
//! operations like filesystem handling, terminal output formatting,
//! messaging, and system-level operations.
//! # Submodules
//! - `colors`: Terminal color codes for formatted output
//! - `fs`: Filesystem helpers for reading, writing, creating, and deleting files/folders
//! - `msgs`: Message utilities for displaying errors, warnings, and info
//! - `sys`: System-level helpers such as argument parsing and shell command execution

/// Terminal color codes
pub mod colors; // Provides pre-defined color constants for terminal output

/// Filesystem helper functions
pub mod fs; // Read/write files, create/delete files/folders safely

/// Messaging utilities
pub mod msgs; // Display errors, info, and other messages consistently

/// System-level helpers
pub mod sys; // Shell commands, argument parsing, and other OS-level utilities
