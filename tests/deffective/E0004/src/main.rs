enum Terminator {
    HastaLaVistaBaby,
    TalkToMyHand,
}

fn main() {
    let x = Terminator::HastaLaVistaBaby;

    match x {
        // error: non-exhaustive patterns: `HastaLaVistaBaby` not covered
        Terminator::TalkToMyHand => {}
    }
}
