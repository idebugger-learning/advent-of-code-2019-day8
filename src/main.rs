const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
    let input = include_str!("../data/input.txt");
    let pixels: Vec<_> = input.chars().collect();

    let mut layers = vec![];
    let mut layer_start = 0;
    while layer_start < pixels.len() {
        let mut layer = vec![];
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let index = layer_start + i * WIDTH + j;
                let pixel = pixels[index];
                layer.push(pixel);
            }
        }
        layers.push(layer);
        layer_start = layer_start + WIDTH * HEIGHT;
    }

    let fewest_0s = layers
        .iter()
        .min_by_key(|layer| layer.iter().filter(|&&chr| chr == '0').count())
        .expect("No minimum 0s layer have found");
    println!("fewest_0s: {:?}", fewest_0s);

    let number_of_1s = fewest_0s.iter().filter(|&&chr| chr == '1').count();
    let number_of_2s = fewest_0s.iter().filter(|&&chr| chr == '2').count();

    println!(
        "{} * {} = {}",
        number_of_1s,
        number_of_2s,
        number_of_1s * number_of_2s
    );

    let mut image = vec![];
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let index = i * WIDTH + j;
            let pixel = layers
                .iter()
                .map(|layer| layer[index])
                // .rev()
                .reduce(|acc, color| if acc == '2' { color } else { acc })
                .expect("No pixel was calculated somehow");
            image.push(pixel);
        }
    }

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let index = i * WIDTH + j;
            let char = if image[index] == '0' { ' ' } else { '8' };
            print!("{}", char);
        }
        println!();
    }
}
