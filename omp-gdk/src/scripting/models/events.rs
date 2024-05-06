use omp_codegen::callback;

use crate::players::Player;

use super::ModelDownloadType;

callback!(OnPlayerFinishedDownloading, player:Player);
callback!(OnPlayerRequestDownload, player:Player, model_type:ModelDownloadType, checksum:u32, ->bool);
