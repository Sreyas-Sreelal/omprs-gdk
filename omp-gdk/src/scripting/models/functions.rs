use omp_codegen::native;

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
native!(FindModelFileNameFromCRC,crc: isize, output: mut str,  -> bool);
native!(FindTextureFileNameFromCRC,crc: isize, output: mut str, -> bool);
native!(IsValidCustomModel,modelid: isize, -> bool);
native!(
    GetCustomModelPath,
    modelid: isize,
    dff_path: mut str,
    txd_path: mut str,
    -> bool
);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(AddCharModel);
    load_function!(AddSimpleModel);
    load_function!(AddSimpleModelTimed);
    load_function!(FindModelFileNameFromCRC);
    load_function!(FindTextureFileNameFromCRC);
    load_function!(IsValidCustomModel);
    load_function!(GetCustomModelPath);
}
