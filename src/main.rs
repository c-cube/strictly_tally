
use {
    std::{ fmt, io::Read, error::Error},
};

#[cfg(test)]
mod test;

type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Clone,Copy,Eq,PartialEq,Debug)]
struct CompetitorID(usize);

#[derive(Clone,Copy,Eq,PartialEq,Debug)]
struct JudgeID(usize);

#[derive(Clone,Debug)]
struct Judge {
    id: JudgeID,
    name: String,
}

#[derive(Clone,Debug)]
struct Competitor {
    id: CompetitorID,
    name: String,
    marked: bool, // marked as winner already?
}

/// A cluster of competitors
#[derive(Clone)]
enum CompetitorCluster {
    One(Competitor),
    Tie(Vec<Competitor>)
}

#[derive(Clone,Debug)]
struct Sheet {
    judges: Vec<Judge>,
    competitors: Vec<Competitor>,
    notes: Vec<Vec<u32>>,
    ranked: Option<Vec<CompetitorCluster>>,
}

impl CompetitorCluster {
    pub fn as_one(&self) -> Option<&Competitor> {
        match self {
            CompetitorCluster::One(c) => Some(c),
            CompetitorCluster::Tie(..) => None,
        }
    }
}

/// temporary, to rank competitors
struct RankCompute<'a> {
    judges: &'a [Judge],
    notes: &'a Vec<Vec<u32>>,
    competitors: &'a mut Vec<Competitor>,
}

impl Sheet {
    fn new() -> Self {
        Sheet {
            judges: vec!(), competitors: vec!(),
            notes: vec!(), ranked: None,
        }
    }

    pub fn n_competitors(&self) -> usize { self.competitors.len() }
    pub fn n_judges(&self) -> usize { self.judges.len() }

    pub fn judges(&self) -> impl Iterator<Item=&Judge> {
        self.judges.iter()
    }

    fn add_reader<R: Read>(&mut self, r: &mut R) -> Res<()> {
        let mut rdr = csv::Reader::from_reader(r);

        {
            let h = rdr.headers()?;

            println!("header: {:?} (skip first column)", &h);

            for j in h.iter().skip(1) {
                self.judges.push(Judge {name: j.trim().to_string(), id: JudgeID(self.judges.len())});
                self.notes.push(vec!());
            }
            //println!("judges: {:?}", &self.judges);
        }

        let width = self.n_judges() + 1;
        for row in rdr.records() {
            let row = row?;

            println!("parsed row {:?}", &row);

            if row.len() != width {
                return Err(format!("expected row to have {} fields, got {:?}", width, row).into())
            }

            let name = row[0].trim().to_string();
            let comp = Competitor{
                id: CompetitorID(self.n_competitors()), name, marked: false,
            };
            self.competitors.push(comp);

            for i in 0..self.n_judges() {
                let score: u32 = row[i+1].trim().parse()?;
                self.notes[i].push(score);
            }
        }

        //println!("competitors: {:?}", &self.competitors);

        Ok(())
    }

    pub fn from_reader<R: Read>(r: &mut R) -> Res<Self> {
        let mut sheet = Self::new();
        sheet.add_reader(r)?;
        Ok(sheet)
    }

    pub fn from_file(f: String) -> Res<Self> {
        let mut file = std::fs::File::open(f)?;
        Self::from_reader(&mut std::io::BufReader::new(&mut file))
    }

    pub fn from_str(s: &str) -> Res<Self> {
        Self::from_reader(&mut s.as_bytes())
    }

    /// Rank given competitors using placement ranking.
    fn rank_for(&mut self, comp: &[CompetitorID], mut placement_lvl: u32) -> Vec<CompetitorCluster> {
        let n_j = self.judges.len();
        let n_c = comp.len();
        let majority = (n_j as u32+1) / 2;
        assert!(majority * 2 > n_j as u32);

        let mut res = vec!();

        println!(
            "\n### ranking competitors {:?} (lvl {}, majority at {})",
            comp.iter().map(|c| &self[*c].name).collect::<Vec<_>>(),
            placement_lvl, majority);

        loop {
            assert!(res.len() <= n_c);
            if res.len() == n_c {
                break
            } else if placement_lvl as usize == n_c {
                // tie
                let mut v = vec!();
                for &c in comp.iter() { v.push(self[c].clone()) }
                println!("reached end of placement, declare tie for {:?}", &v);
                res.push(CompetitorCluster::Tie(v));
                break;
            }

            let v = self.n_placements_under(comp, placement_lvl);

            let potential_winners: Vec<_> =
                v.iter().filter(
                    |(c,n)| *n >= majority && !self[*c].marked
                ).collect();

            if potential_winners.len() == 0 {
                placement_lvl += 1;
            } else if potential_winners.len() == 1 {
                let c = &mut self[potential_winners[0].0];
                assert!(! c.marked);
                c.marked =  true;
                println!("winner for this round (by majority): {:?}", c.name);
                res.push(CompetitorCluster::One(c.clone()))
            } else {
                assert!(potential_winners.len() >= 2);
                // several candidates
                println!("several candidates: {:?}",
                         potential_winners.iter().map(|(c,n)| (&self[*c].name,n)).collect::<Vec<_>>());

                // method 1: biggest majority
                let biggest_majority = potential_winners.iter().map(|(_,n)| *n).max().unwrap();
                let competitors_with_biggest_majority: Vec<_> =
                    potential_winners
                        .iter().filter(|(_,n)| *n == biggest_majority)
                        .map(|(c,_)| *c) .collect();

                assert!(competitors_with_biggest_majority.len() >= 1);
                if competitors_with_biggest_majority.len() == 1 {
                    let c = &mut self[competitors_with_biggest_majority[0]];

                    assert!(! c.marked);
                    c.marked =  true;
                    println!(
                        "winner for this round (biggest majority of {}): {:?}",
                        biggest_majority, c.name);
                    res.push(CompetitorCluster::One(c.clone()));
                    continue;
                } else {
                    // tie breaking 1

                    let sums = self.sum_placements_under(
                        competitors_with_biggest_majority.as_slice(), placement_lvl);
                    let smallest_sum =
                        sums.iter().map(|(_,n)| n).min().unwrap();
                    println!("several candidates have majority, smallest sum is {}", smallest_sum);

                    let competitors_with_smallest_sum: Vec<_> =
                        sums.iter().filter(|(_,n)| n == smallest_sum).map(|(c,_)| *c).collect();

                    assert!(competitors_with_smallest_sum.len() >= 1);
                    if competitors_with_smallest_sum.len() == 1 {
                        let c = &mut self[competitors_with_smallest_sum[0]];

                        assert!(! c.marked);
                        c.marked =  true;
                        println!(
                            "winner for this round (with smallest sum {}): {:?}",
                            smallest_sum, c.name);
                        res.push(CompetitorCluster::One(c.clone()));
                        continue;
                    } else {
                        println!(
                            "competitors {:?} have same smallest sum {}, need to go deeper",
                            competitors_with_smallest_sum.iter().map(|c| &self[*c].name).collect::<Vec<_>>(),
                            smallest_sum);

                        // sort only these competitors, at next level
                        let r = self.rank_for(&competitors_with_smallest_sum, placement_lvl+1);
                        res.extend_from_slice(&r);
                    }
                }
            }
        }

        res
    }

    /// Number of scores that are `≤ placement_lvl, for each competitor
    fn n_placements_under(&self, comp: &[CompetitorID], placement_lvl: u32) -> Vec<(CompetitorID,u32)> {
        println!("\n## computing {}-placements", placement_lvl);
        let mut v = vec!();
        for &c in comp.iter() {
            let mut n = 0u32;
            for j in self.judges.iter().map(|j| j.id) {
                if self[(j, c)] <= placement_lvl {
                    n += 1;
                }
            }
            v.push((c,n));
        }

        println!("{}-placements: {:?}", placement_lvl, &v);
        v
    }

    /// Sum of scores that are `≤ placement_lvl`, for each competitor
    fn sum_placements_under(&self, comp: &[CompetitorID], placement_lvl: u32) -> Vec<(CompetitorID,u32)> {
        println!("\n### computing {}-Σ-placements", placement_lvl);
        let mut v = vec!();
        for &c in comp.iter() {
            let mut n = 0u32;
            for j in self.judges.iter().map(|j| j.id) {
                let x = self[(j,c)];
                if x <= placement_lvl {
                    n += x;
                }
            }
            v.push((c,n));
        }

        println!("{}-Σ-placements: {:?}", placement_lvl, &v);
        v
    }

    /// Rank competitors using placement ranking.
    pub fn rank(&mut self) -> &Vec<CompetitorCluster> {
        if let Some(ref r) = self.ranked {
            return r
        }

        let n_j = self.judges.len();
        let majority = (n_j as u32+1) / 2;
        println!("## ranking competitors (majority at {})", majority);

        let Sheet {notes, judges, competitors, ..} = self;
        let comps: Vec<_> = competitors.iter().map(|c| c.id).collect();

        let res = self.rank_for(&comps, 1);

        self.ranked = Some(res);
        self.ranked.as_ref().unwrap()
    }
}

impl<'a> RankCompute<'a> {
}

impl std::ops::Index<JudgeID> for Sheet {
    type Output = Judge;
    fn index(&self, idx: JudgeID) -> &Self::Output {
        &self.judges[idx.0 as usize]
    }
}

impl std::ops::Index<CompetitorID> for Sheet {
    type Output = Competitor;
    fn index(&self, idx: CompetitorID) -> &Self::Output {
        &self.competitors[idx.0 as usize]
    }
}

impl std::ops::IndexMut<CompetitorID> for Sheet {
    fn index_mut(&mut self, idx: CompetitorID) -> &mut Self::Output {
        &mut self.competitors[idx.0 as usize]
    }
}

impl std::ops::Index<(JudgeID, CompetitorID)> for Sheet {
    type Output = u32;
    fn index(&self, idx: (JudgeID, CompetitorID)) -> &Self::Output {
        let (j,c) = idx;
        &self.notes[j.0 as usize][c.0 as usize]
    }
}

impl fmt::Display for Sheet {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "\n=== Sheet ===\n{:-15}", "name")?;
        for j in self.judges() { write!(out, " {:-3}", j.name)? }
        for c in self.competitors.iter() {
            write!(out, "\n")?;
            write!(out, "{:-15}", c.name)?;
            for i in 0..self.n_judges() {
                write!(out, " {:-3}", self.notes[i][c.id.0 as usize])?;
            }
        }
        write!(out, "\n=============\n")?;
        Ok(())
    }
}

impl fmt::Display for CompetitorCluster {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompetitorCluster::One(c) => write!(out, "{:?}", c.name),
            CompetitorCluster::Tie(v) => {
                write!(out, "{{")?;
                for (i,c) in v.iter().enumerate() {
                    if i>0 { write!(out, ", ")? }
                    write!(out, "{:?}", c.name);
                }
                write!(out, "}}")
            },
        }
    }
}

impl fmt::Debug for CompetitorCluster {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, out) // delegate to display
    }
}



fn main() -> Res<()> {
    let file = std::env::args().skip(1).next().expect("please give a file");
    let mut sheet = Sheet::from_file(file)?;
    println!("parsed sheet {}", &sheet);

    let ranked = sheet.rank();
    println!("ranked: {:?}", &ranked);
    Ok(())
}
