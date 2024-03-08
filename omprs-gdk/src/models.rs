use omprs_codegen::native;
use std::ffi::{c_char, CStr};

native!(AddCharModel,baseid: isize, newid: isize, dffname: str, txdname: str, -> bool);
native!(
    AddSimpleModel,
    virtualworld: isize,
    baseid: isize,
    newid: isize,
    dffname: str,
    txdname: str,
    -> bool
);
native!(
    AddSimpleModelTimed,
    virtualworld: isize,
    baseid: isize,
    newid: isize,
    dffname: str,
    txdname: str,
    timeon: isize,
    timeoff: isize,
 -> bool
);
native!(GetPlayerCustomSkin,playerid: isize,-> isize);
native!(RedirectDownload,playerid: isize, url: str, -> bool);
native!(FindModelFileNameFromCRC,crc: isize, output: mut str, output_len: usize, -> bool);
native!(FindTextureFileNameFromCRC,crc: isize, output: mut str, output_len: usize, -> bool);
native!(IsValidCustomModel,modelid: isize, -> bool);
native!(
    GetCustomModelPath,
    modelid: isize, 
    dff_path: mut str,
    dff_path_len: usize,
    txd_path: mut str,
    txd_path_len: usize,
    -> bool
);
