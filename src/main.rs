use yansi::Paint;

fn main() {
    println!(
        "Testing, {}, {}, {}!",
        "Ready".bold(),
        "Set".black().on_yellow().invert().italic(),
        "STOP".white().on_red().bright().underline().bold()
    );
}
