use bevy::prelude::*;

use crate::table::components::Chips;

#[derive(Component, Debug)]
pub struct MainCamera;

#[derive(Component, Debug)]
pub struct WindowBackground;

#[derive(Component, Debug)]
pub struct TableTop;

#[derive(Component, Debug)]
pub struct TableRoot;

#[derive(Component, Debug)]
pub struct BankrollText;

#[derive(Component, Debug)]
pub struct StackText;

#[derive(Component, Debug)]
pub struct MovedChipsText;

#[derive(Component, Debug, Clone)]
pub struct BoardCardIndex(pub usize);

#[derive(Component, Debug, Clone)]
pub struct HoleCardIndex(pub usize);

#[derive(Component, Debug)]
pub struct InfoMessage;

#[derive(Component, Debug)]
pub struct DealerButtonText;

#[derive(Component, Debug)]
pub struct CardFace;

#[derive(Component, Debug)]
pub struct CardBack;

#[derive(Component, Debug)]
pub struct CardRankText;

#[derive(Component, Debug)]
pub struct PotText;

#[derive(Component, Debug)]
pub struct PooledPotText;

#[derive(Component, Debug)]
pub struct InActionIndicator;

#[derive(Component, Debug)]
pub struct MoveControls;

#[derive(Component, Debug)]
pub struct ButtonRect(pub Rect);

#[derive(Component, Debug)]
pub enum MoveButtons {
    FoldButton,
    CallButton(Chips),
    RaiseButton(Chips),
}

#[derive(Component, Debug)]
pub struct FoldButton;

#[derive(Component, Debug)]
pub struct RaiseButtonText;

#[derive(Component, Debug)]
pub struct CallButtonText;

#[derive(Component, Debug)]
pub struct PauseButton;

#[derive(Component, Debug)]
pub struct PauseButtonText;

#[derive(Component, Debug)]
pub struct DummyLabel;
