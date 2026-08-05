pub fn part_1(input: &str) -> u32 {
    // let shapes =
    //
    //

    let regions = input.lines().map(|l| {
        let (dimensions, quantities_str) = l.split_once(": ").unwrap();

        // Parse dimensions (e.g., "12x5")
        let (w, h) = dimensions
            .split_once('x')
            .map(|(w_str, h_str)| (w_str.parse::<u32>().unwrap(), h_str.parse::<u32>().unwrap()))
            .unwrap();

        // Parse quantities (e.g., "1 0 1 0 2 2")
        let quantities: Vec<u32> = quantities_str
            .split_whitespace()
            .map(|q| q.parse::<u32>().unwrap())
            .collect();

        (w, h, quantities)
    });

    todo!()
}

pub fn part_2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
    #[test]
    fn test_part_1() {
        // let input = parse(SAMPLE_INPUT);
        // assert_eq!(4277556, part_1(&input))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3263827, part_2(SAMPLE_INPUT))
    }
}
