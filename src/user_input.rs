/*!
 * Functions related to any user interaction
 *
 * # Author
 * Doran Kayoumi <doran.kayoumi@heig-vd.ch>
 */

use read_input::prelude::*;

/// Ask the user to enter an email address
pub fn ask_for_email() -> String {
    input().msg("Username : ").get()
}

/// Ask the user for a password without checking the policy
pub fn ask_for_password() -> String {
    input().msg("Password : ").get()
}
