use std::collections::HashSet;

#[derive(Clone)]
struct Sudoku {
    base: usize,
    size: usize,
    puzzle: Vec<Vec<usize>>,
}

impl Sudoku {
    fn check(&self,loc_1:usize,loc_2:usize) -> HashSet<usize> {
        let horizontal :HashSet<usize> = self.puzzle[loc_1]
            .iter()
            .map(|&x| x)
            .collect();
        let vertical:HashSet<usize>  = (0..self.size)
            .map(|x| self.puzzle[x][loc_2]).collect();
        let block:HashSet<usize> = (0..self.size)
            .map(|x| self.puzzle[x%self.base + (loc_1/self.base)*self.base][x/self.base+(loc_2/self.base)*self.base])
            .collect();
        (1..(self.size + 1))
            .filter(|x| !horizontal.contains(x))
            .filter(|x| !vertical.contains(x))
            .filter(|x| !block.contains(x))
            .collect()
    }

    fn solve_trivial(&mut self) -> Option<bool> {
        let mut changed = true;
        let mut solved = false;
        while changed {
            changed = false;
            solved = true;
            for x in 0..self.size {
                for y in 0..self.size {
                    if self.puzzle[x][y] == 0 { 
                        solved = false;
                        let mut possible = self.check(x,y);
                        if possible.len() == 1 {
                            self.puzzle[x][y] = possible.drain().next().unwrap(); 
                            changed = true
                        } else if possible.len() == 0 {
                            return None
                        }
                    }
                }
            }
        }
        return Some(solved)
    }
    fn solve(&mut self) -> Option<()> {
        match self.clone().solve_help() {
            Some(x) => self.puzzle = x.puzzle,
            None => return None
        }
        return Some(())
    }

    fn solve_help(mut self) -> Option<Self> {
        match self.solve_trivial() {
            Some(true) => return Some(self),
            Some(_) => 10,
            None => return None
        };
        let mut min = self.size + 1;
        let mut min_x = 0;
        let mut min_y = 0;
        let mut min_set = HashSet::new();
        for x in 0..self.size {
            for y in 0..self.size {
                if self.puzzle[x][y] == 0 {
                    let mut possible = self.check(x,y);
                    if possible.len() < min {
                        min = possible.len();
                        min_x = x;
                        min_y = y;
                        min_set = possible;
                    }
                }
            }
        }
        for option in min_set {
            let mut temp = self.clone(); 
            temp.puzzle[min_x][min_y] = option;
            match temp.solve_help() {
                Some(x) => return Some(x),
                None => 21212121,
            };
        }
        return None 
    }
}


fn main() {
    println!("Hello, world!");
    let mut sudoku = Sudoku {
        base :3,
        size: 9,
        puzzle: vec![
            vec![0,0,7,0,0,0,0,4,0],
            vec![4,0,0,2,0,0,5,0,0],
            vec![1,8,0,0,4,0,0,0,0],
            vec![0,5,0,0,6,0,7,0,0],
            vec![0,3,0,5,0,8,0,1,0],
            vec![0,0,1,0,2,0,0,3,0],
            vec![0,0,0,0,8,0,0,6,4],
            vec![0,0,6,0,0,7,0,0,2],
            vec![0,2,0,0,0,0,3,0,0]
        ]
    };
    match sudoku.solve() {
        Some(_) => println!("solvable"),
        None => println!("unsolvable")
    }
    println!("{:?}",sudoku.puzzle);
}
