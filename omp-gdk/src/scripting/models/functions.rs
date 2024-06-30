use crate::players::Player;
use omp_codegen::native;
use std::ffi::c_void;

native!(CustomModel_AddCharModel, baseid: i32, newid: i32, dff: str, textureLibrary: str, -> bool);
native!(CustomModel_AddSimpleModel, virtualWorld: i32, baseid: i32, newid: i32, dff: str, textureLibrary: str, -> bool);
native!(CustomModel_AddSimpleModelTimed, virtualWorld: i32, baseid: i32, newid: i32, dff: str, textureLibrary: str, timeOn: i32, timeOff: i32, -> bool);
native!(CustomModel_RedirectDownload, player: struct Player, url: str, -> bool);
native!(CustomModel_FindModelFileNameFromCRC, crc: i32, output: mut str, output_len: usize, -> i32);
native!(CustomModel_IsValid, modelId: i32, -> bool);
native!(CustomModel_GetPath, modelId: i32, dffPath: mut str, dffPath_len: usize, txdPath: mut str, txdPath_len: usize, -> bool);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(CustomModel_AddCharModel);
    load_function!(CustomModel_AddSimpleModel);
    load_function!(CustomModel_AddSimpleModelTimed);
    load_function!(CustomModel_RedirectDownload);
    load_function!(CustomModel_FindModelFileNameFromCRC);
    load_function!(CustomModel_IsValid);
    load_function!(CustomModel_GetPath);
}
