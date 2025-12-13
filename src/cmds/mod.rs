//! Command Management Module for cmdcreate
//!
//! This is the root module for all command-related functionality in cmdcreate.
//! It provides a comprehensive set of submodules for managing custom commands
//! in the system, including creation, modification, listing, and maintenance.
//!
//! # Module Structure
//! - Core Command Operations:
//!   - `create`: Command creation and installation
//!   - `remove`: Command removal and cleanup
//!   - `edit`: Command content modification
//!   - `rename`: Command name changes
//!   - `favorite`: Favorite command management
//!   - `repair`: Repair broken commands
//!
//! - Information Display:
//!   - `display`: Show command contents
//!   - `list`: Show all installed commands
//!   - `search`: Find specific commands
//!
//! - System Operations:
//!   - `upgrader`: System update functionality
//!   - `backup`: Command backup and restore
//!
//! - Internal Utilities:
//!   - `tools`: Shared utilities for command operations

// Core command management modules
pub mod create; // Command creation functionality
pub mod display; // Command content display
pub mod edit; // Command editing operations
pub mod favorite; // Favorite command management
pub mod list; // Command listing utility
pub mod remove; // Command removal functionality
pub mod rename; // Command renaming operations
pub mod repair; // Command repairing functionality
pub mod search; // Command search functionality
pub mod upgrader; // Update management // Repair broken commands

// Backup and restore functionality
pub mod backup; // Command backup/restore operations

// Internal utilities
mod tools; // Shared command operation utilities
