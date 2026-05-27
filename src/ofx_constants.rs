#![allow(dead_code, non_upper_case_globals)]

/** @brief Status code indicating all was fine */
pub const kOfxStatOK: i32 = 0;

/** @brief Status error code for a failed operation. */
pub const kOfxStatFailed: i32 = 1;

/** @brief Status error code for a fatal error

 Only returned in the case where the plug-in or host cannot continue to function and needs to be restarted.
*/
pub const kOfxStatErrFatal: i32 = 2;

/** @brief Status error code for an operation on or request for an unknown object */
pub const kOfxStatErrUnknown: i32 = 3;

/** @brief Status error code returned by plug-ins when they are missing host functionality, either an API or some optional functionality (eg: custom params).

   Plug-Ins returning this should post an appropriate error message stating what they are missing.
*/
pub const kOfxStatErrMissingHostFeature: i32 = 4;

/** @brief Status error code for an unsupported feature/operation */
pub const kOfxStatErrUnsupported: i32 = 5;

/** @brief Status error code for an operation attempting to create something that exists */
pub const kOfxStatErrExists: i32 = 6;

/** @brief Status error code for an incorrect format */
pub const kOfxStatErrFormat: i32 = 7;

/** @brief Status error code indicating that something failed due to memory shortage */
pub const kOfxStatErrMemory: i32 = 8;

/** @brief Status error code for an operation on a bad handle */
pub const kOfxStatErrBadHandle: i32 = 9;

/** @brief Status error code indicating that a given index was invalid or unavailable */
pub const kOfxStatErrBadIndex: i32 = 10;

/** @brief Status error code indicating that something failed due an illegal value */
pub const kOfxStatErrValue: i32 = 11;

/** @brief OfxStatus returned indicating a 'yes' */
pub const kOfxStatReplyYes: i32 = 12;

/** @brief OfxStatus returned indicating a 'no' */
pub const kOfxStatReplyNo: i32 = 13;

/** @brief OfxStatus returned indicating that a default action should be performed */
pub const kOfxStatReplyDefault: i32 = 14;
