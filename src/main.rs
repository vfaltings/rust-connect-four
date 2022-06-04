fn main() {
    setup();

    loop {
        play_one_round();
        check_if_over();
        display_board();
    }
}
