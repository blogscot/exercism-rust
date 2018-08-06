use std::fmt;

type Domino = (usize, usize);

pub fn chain(input: &[Domino]) -> Option<Vec<Domino>> {
  if input.is_empty() {
    return Some(Vec::new());
  } else if input.len() == 1 {
    let (left, right) = input[0];
    if left == right {
      return Some(input.to_vec());
    } else {
      return None;
    }
  }
  for index in 0..input.len() {
    let mut doms: Dominoes = Dominoes::new(input);
    let sequence = match doms.sequence_dominoes(index) {
      Some(value) => value,
      None => return None,
    };
    if sequence.available.is_empty() && sequence.start_and_end_dots_are_equal() {
      return Some(sequence.allocated);
    }
  }
  None
}

#[derive(PartialEq, Clone)]
struct Dominoes {
  available: Vec<Domino>,
  allocated: Vec<Domino>,
}

impl Dominoes {
  fn new(input: &[Domino]) -> Self {
    let available = input.to_vec();
    let allocated = Vec::new();
    Dominoes {
      available,
      allocated,
    }
  }
  fn sequence_dominoes(&self, start: usize) -> Option<Self> {
    let first_dom = self.available[start];
    Self::sequence_helper(first_dom, self.available.clone(), self.allocated.clone())
  }
  fn sequence_helper(
    current_dom: Domino,
    available: Vec<Domino>,
    allocated: Vec<Domino>,
  ) -> Option<Self> {
    let neighbours = Self::find_neighbours(&available, current_dom.1);
    if neighbours.is_empty() {
      return Some(Dominoes {
        available,
        allocated,
      });
    }
    let results: Vec<Option<Self>> = neighbours
      .into_iter()
      // Recurse the list of dominoes until there is no more
      // left to play
      .map(|neighbour| {
        let mut next_domino = available[neighbour];
        if current_dom.1 != next_domino.0 {
          next_domino = Self::reverse_domino(next_domino);
        }
        let Dominoes {
          available,
          allocated,
        } = Self::play_domino(&next_domino, &available, &allocated);
        Self::sequence_helper(next_domino, available, allocated)
      })
      // Remove non-solutions
      .filter(|result| match result {
        Some(value) => value.available.is_empty(),
        None => false,
      })
      .collect();
    // Return the first solution
    match results.get(0) {
      None => None,
      Some(value) => Some(value.clone().unwrap()),
    }
  }
  /**
   * Returns a list of neighbours for the given domino.
   * Assumes that the given domino has already been removed
   * from the list of candidates.
   */
  fn find_neighbours(available: &[Domino], dots: usize) -> Vec<usize> {
    let mut matches = vec![];
    for (index, domino) in available.iter().enumerate() {
      let (left_dots, right_dots) = domino;
      if *left_dots == dots || *right_dots == dots {
        matches.push(index);
      }
    }
    matches
  }
  /**
   * Moves the given domino from available to allocated resource.
   */
  fn play_domino(domino: &Domino, available: &[Domino], allocated: &[Domino]) -> Self {
    let mut allocated = allocated.to_vec();
    allocated.push(*domino);
    let reversed_dom = Self::reverse_domino(*domino);
    let position = available
      .iter()
      .position(|&dom| dom == *domino || dom == reversed_dom)
      .unwrap();
    let available = available
      .iter()
      .enumerate()
      .filter_map(|(index, &value)| if index != position { Some(value) } else { None })
      .collect();
    Dominoes {
      available,
      allocated,
    }
  }
  fn start_and_end_dots_are_equal(&self) -> bool {
    let allocation = self.allocated.to_vec();
    let first_dom = allocation.first().unwrap();
    let last_dom = allocation.last().unwrap();
    first_dom.0 == last_dom.1
  }
  fn reverse_domino(domino: Domino) -> Domino {
    (domino.1, domino.0)
  }
}

impl<'a> fmt::Debug for Dominoes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut output = Vec::new();
    for domino in &self.available {
      output.push(domino);
    }
    write!(f, "Available: {:?}", output)?;
    output.clear();
    for link in &self.allocated {
      output.push(link);
    }
    write!(f, " Allocated: {:?}", output)
  }
}
