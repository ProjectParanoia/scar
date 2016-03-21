pub struct GlobalState {
    hiscores : HashMap<u32, StageData>
}

pub struct StageData {
    unlocked : bool,
    hiscore: u32,
    tries: u32,
}
