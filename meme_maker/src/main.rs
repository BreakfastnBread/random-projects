use rand::{self, Rng};

fn main() {
    let prompts = [
    "when I shid myself",
    "when I have school tmr",
    "when I go to dinner",
    "when my family tells me i'm adopted",
    "when I see man-made horrors beyond my comprehension",
    "when I put on moisturizer",
    "when i stub my toe",
    "when spotify plays a banger",
    "when i get my bread up"
    ];
    let reaction: [char; 5] = ['😏', '🗿', '💀', '👽', '💯'];
    let prompts = prompts[
        rand::thread_rng()
        .gen_range(0..(
            prompts.len() - 1
        )
        )
    ];
    let reaction = reaction[
        rand::thread_rng()
        .gen_range(0..4)
    ];
    println!("mfw {prompts}:{reaction}");
}
