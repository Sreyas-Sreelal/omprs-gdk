use omp_codegen::native;
use std::ffi::c_void;

use crate::{
    players::Player,
    types::colour::Colour,
    types::vector::{Vector2, Vector3},
};

use super::{PlayerTextDraw, TextDraw, TextDrawAlignmentTypes, TextDrawStyle};

native!(TextDrawCreate, position: Vector2, text: str, -> struct TextDraw);
native!(TextDrawDestroy, textdraw: struct TextDraw);
native!(IsTextDrawShownForPlayer, textdraw: struct TextDraw, player: struct Player, -> bool);
native!(TextDrawSetLetterSize, textdraw: struct TextDraw, size: Vector2);
native!(TextDrawSetTextSize, textdraw: struct TextDraw, size: Vector2);
native!(TextDrawSetAlignment, textdraw: struct TextDraw, alignment: TextDrawAlignmentTypes);
native!(TextDrawSetColor, textdraw: struct TextDraw, colour: Colour);
native!(TextDrawUseBox, textdraw: struct TextDraw, use_box: bool);
native!(TextDrawSetBoxColor, textdraw: struct TextDraw, colour: Colour);
native!(TextDrawSetShadow, textdraw: struct TextDraw, size: isize);
native!(TextDrawSetOutline, textdraw: struct TextDraw, size: isize);
native!(TextDrawSetBackgroundColor, textdraw: struct TextDraw, colour: Colour);
native!(TextDrawSetStyle, textdraw: struct TextDraw, font: TextDrawStyle);
native!(TextDrawSetProportional, textdraw: struct TextDraw, set: bool);
native!(TextDrawSetSelectable, textdraw: struct TextDraw, set: bool);
native!(TextDrawShowForPlayer, textdraw: struct TextDraw, player: struct Player);
native!(TextDrawHideForPlayer, textdraw: struct TextDraw, player: struct Player);
native!(TextDrawShowForAll, textdraw: struct TextDraw);
native!(TextDrawHideForAll, textdraw: struct TextDraw);
native!(TextDrawSetString, textdraw: struct TextDraw, text: str);
native!(TextDrawSetPreviewModel, textdraw: struct TextDraw, model: isize);
native!(TextDrawSetPreviewRotation, textdraw: struct TextDraw, rotation: Vector3, zoom: f32);
native!(TextDrawSetPreviewVehColour, textdraw: struct TextDraw, colour1: isize, colour2: isize);
native!(TextDrawSetPos, textdraw: struct TextDraw, pos: Vector2);
native!(TextDrawGetString, textdraw: struct TextDraw, text: mut str);
native!(TextDrawGetLetterSize, textdraw: struct TextDraw, size: mut Vector2);
native!(TextDrawGetTextSize, textdraw: struct TextDraw, size: mut Vector2);
native!(TextDrawGetPos, textdraw: struct TextDraw, pos: mut Vector2);
native!(TextDrawGetColor, textdraw: struct TextDraw,colour: mut Colour);
native!(TextDrawGetBoxColor, textdraw: struct TextDraw, colour: mut Colour);
native!(TextDrawGetBackgroundColor, textdraw: struct TextDraw, colour: mut Colour);
native!(TextDrawGetShadow, textdraw: struct TextDraw, -> isize);
native!(TextDrawGetOutline, textdraw: struct TextDraw, -> isize);
native!(TextDrawGetStyle, textdraw: struct TextDraw, -> isize);
native!(TextDrawIsBox, textdraw: struct TextDraw, -> bool);
native!(TextDrawIsProportional, textdraw: struct TextDraw, -> bool);
native!(TextDrawIsSelectable, textdraw: struct TextDraw, -> bool);
native!(TextDrawGetAlignment, textdraw: struct TextDraw, -> TextDrawAlignmentTypes);
native!(TextDrawGetPreviewModel, textdraw: struct TextDraw, -> isize);
native!(TextDrawGetPreviewRotation, textdraw: struct TextDraw, rotation: mut Vector3, zoom: mut f32);
native!(TextDrawGetPreviewVehColour, textdraw: struct TextDraw, colour1: mut isize, colour2: mut isize);
native!(TextDrawSetStringForPlayer, textdraw: struct TextDraw, player: struct Player,text: str);
native!(TextDrawGetID, textdraw: struct TextDraw, -> isize);
native!(TextDrawFromID, textdrawid: isize, -> struct TextDraw);

// player textdraws

native!(CreatePlayerTextDraw, player: struct Player, position: Vector2, text: str, -> struct PlayerTextDraw);
native!(PlayerTextDrawDestroy, player: struct Player, textdraw: struct PlayerTextDraw);
native!(IsPlayerTextDrawShown, textdraw: struct PlayerTextDraw, -> bool);
native!(PlayerTextDrawSetLetterSize, textdraw: struct PlayerTextDraw, size: Vector2);
native!(PlayerTextDrawSetTextSize, textdraw: struct PlayerTextDraw, size: Vector2);
native!(PlayerTextDrawAlignment, textdraw: struct PlayerTextDraw, alignment: TextDrawAlignmentTypes);
native!(PlayerTextDrawColor, textdraw: struct PlayerTextDraw, colour: Colour);
native!(PlayerTextDrawUseBox, textdraw: struct PlayerTextDraw, box_use: bool);
native!(PlayerTextDrawSetBoxColor, textdraw: struct PlayerTextDraw, colour: Colour);
native!(PlayerTextDrawSetShadow, textdraw: struct PlayerTextDraw, size: isize);
native!(PlayerTextDrawSetOutline, textdraw: struct PlayerTextDraw, size: isize);
native!(PlayerTextDrawBackgroundColor, textdraw: struct PlayerTextDraw, colour: Colour);
native!(PlayerTextDrawSetStyle, textdraw: struct PlayerTextDraw, font: TextDrawStyle);
native!(PlayerTextDrawSetProportional, textdraw: struct PlayerTextDraw, set: bool);
native!(PlayerTextDrawSetSelectable, textdraw: struct PlayerTextDraw, set: bool);
native!(PlayerTextDrawShow, textdraw: struct PlayerTextDraw);
native!(PlayerTextDrawHide, textdraw: struct PlayerTextDraw);
native!(PlayerTextDrawSetString, textdraw: struct PlayerTextDraw, text: str);
native!(PlayerTextDrawSetPreviewModel, textdraw: struct PlayerTextDraw, model: isize);
native!(PlayerTextDrawSetPreviewRotation, textdraw: struct PlayerTextDraw, rotation: Vector3, zoom: f32);
native!(PlayerTextDrawSetPreviewVehColour, textdraw: struct PlayerTextDraw, colour1: isize, colour2: isize);
native!(PlayerTextDrawSetPos, textdraw: struct PlayerTextDraw, pos: Vector2);
native!(PlayerTextDrawGetString, textdraw: struct PlayerTextDraw, text: mut str);
native!(PlayerTextDrawGetLetterSize, textdraw: struct PlayerTextDraw, size: mut Vector2);
native!(PlayerTextDrawGetTextSize, textdraw: struct PlayerTextDraw, size: mut Vector2);
native!(PlayerTextDrawGetPos, textdraw: struct PlayerTextDraw, pos: mut Vector2);
native!(PlayerTextDrawGetColor, textdraw: struct PlayerTextDraw,colour: mut Colour);
native!(PlayerTextDrawGetBoxColor, textdraw: struct PlayerTextDraw,colour: mut Colour);
native!(PlayerTextDrawGetBackgroundColour, textdraw: struct PlayerTextDraw, colour: mut Colour);
native!(PlayerTextDrawGetShadow, textdraw: struct PlayerTextDraw, -> isize);
native!(PlayerTextDrawGetOutline, textdraw: struct PlayerTextDraw, -> isize);
native!(PlayerTextDrawGetStyle, textdraw: struct PlayerTextDraw, -> isize);
native!(PlayerTextDrawIsBox, textdraw: struct PlayerTextDraw, -> bool);
native!(PlayerTextDrawIsProportional, textdraw: struct PlayerTextDraw, -> bool);
native!(PlayerTextDrawIsSelectable, textdraw: struct PlayerTextDraw, -> bool);
native!(PlayerTextDrawGetAlignment, textdraw: struct PlayerTextDraw, -> TextDrawAlignmentTypes);
native!(PlayerTextDrawGetPreviewModel, textdraw: struct PlayerTextDraw, -> isize);
native!(PlayerTextDrawGetPreviewRotation, textdraw: struct PlayerTextDraw, rotation: mut Vector3, zoom: mut f32);
native!(PlayerTextDrawGetPreviewVehColour, textdraw: struct PlayerTextDraw, colour1: mut isize, colour2: mut isize);
native!(PlayerTextDrawGetID, textdraw: struct PlayerTextDraw, -> isize);
native!(PlayerTextDrawFromID, textdrawid: isize,player: struct Player, -> struct PlayerTextDraw);

pub fn load_functions() {
    load_function!(TextDrawCreate);
    load_function!(TextDrawDestroy);
    load_function!(IsTextDrawShownForPlayer);
    load_function!(TextDrawSetLetterSize);
    load_function!(TextDrawSetTextSize);
    load_function!(TextDrawSetAlignment);
    load_function!(TextDrawSetColor);
    load_function!(TextDrawUseBox);
    load_function!(TextDrawSetBoxColor);
    load_function!(TextDrawSetShadow);
    load_function!(TextDrawSetOutline);
    load_function!(TextDrawSetBackgroundColor);
    load_function!(TextDrawSetStyle);
    load_function!(TextDrawSetProportional);
    load_function!(TextDrawSetSelectable);
    load_function!(TextDrawShowForPlayer);
    load_function!(TextDrawHideForPlayer);
    load_function!(TextDrawShowForAll);
    load_function!(TextDrawHideForAll);
    load_function!(TextDrawSetString);
    load_function!(TextDrawSetPreviewModel);
    load_function!(TextDrawSetPreviewRotation);
    load_function!(TextDrawSetPreviewVehColour);
    load_function!(TextDrawSetPos);
    load_function!(TextDrawGetString);
    load_function!(TextDrawGetLetterSize);
    load_function!(TextDrawGetTextSize);
    load_function!(TextDrawGetPos);
    load_function!(TextDrawGetColor);
    load_function!(TextDrawGetBoxColor);
    load_function!(TextDrawGetBackgroundColor);
    load_function!(TextDrawGetShadow);
    load_function!(TextDrawGetOutline);
    load_function!(TextDrawGetStyle);
    load_function!(TextDrawIsBox);
    load_function!(TextDrawIsProportional);
    load_function!(TextDrawIsSelectable);
    load_function!(TextDrawGetAlignment);
    load_function!(TextDrawGetPreviewModel);
    load_function!(TextDrawGetPreviewRotation);
    load_function!(TextDrawGetPreviewVehColour);
    load_function!(TextDrawSetStringForPlayer);
    load_function!(TextDrawGetID);
    load_function!(TextDrawFromID);

    // player textdraws

    load_function!(CreatePlayerTextDraw);
    load_function!(PlayerTextDrawDestroy);
    load_function!(IsPlayerTextDrawShown);
    load_function!(PlayerTextDrawSetLetterSize);
    load_function!(PlayerTextDrawSetTextSize);
    load_function!(PlayerTextDrawAlignment);
    load_function!(PlayerTextDrawColor);
    load_function!(PlayerTextDrawUseBox);
    load_function!(PlayerTextDrawSetBoxColor);
    load_function!(PlayerTextDrawSetShadow);
    load_function!(PlayerTextDrawSetOutline);
    load_function!(PlayerTextDrawBackgroundColor);
    load_function!(PlayerTextDrawSetStyle);
    load_function!(PlayerTextDrawSetProportional);
    load_function!(PlayerTextDrawSetSelectable);
    load_function!(PlayerTextDrawShow);
    load_function!(PlayerTextDrawHide);
    load_function!(PlayerTextDrawSetString);
    load_function!(PlayerTextDrawSetPreviewModel);
    load_function!(PlayerTextDrawSetPreviewRotation);
    load_function!(PlayerTextDrawSetPreviewVehColour);
    load_function!(PlayerTextDrawSetPos);
    load_function!(PlayerTextDrawGetString);
    load_function!(PlayerTextDrawGetLetterSize);
    load_function!(PlayerTextDrawGetTextSize);
    load_function!(PlayerTextDrawGetPos);
    load_function!(PlayerTextDrawGetColor);
    load_function!(PlayerTextDrawGetBoxColor);
    load_function!(PlayerTextDrawGetBackgroundColour);
    load_function!(PlayerTextDrawGetShadow);
    load_function!(PlayerTextDrawGetOutline);
    load_function!(PlayerTextDrawGetStyle);
    load_function!(PlayerTextDrawIsBox);
    load_function!(PlayerTextDrawIsProportional);
    load_function!(PlayerTextDrawIsSelectable);
    load_function!(PlayerTextDrawGetAlignment);
    load_function!(PlayerTextDrawGetPreviewModel);
    load_function!(PlayerTextDrawGetPreviewRotation);
    load_function!(PlayerTextDrawGetPreviewVehColour);
    load_function!(PlayerTextDrawGetID);
    load_function!(PlayerTextDrawFromID);
}