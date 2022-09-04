// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub struct Error;

// No Sharps or Flats: C major a minor
// Use Sharps: G, D, A, E, B, F# major e, b, f#, c#, g#, d# minor
// Use Flats: F, Bb, Eb, Ab, Db, Gb major d, g, c, f, bb, eb minor
const SHARPS: [&str; 14] = [
    "C", "a", "G", "D", "A", "E", "B", "F#", "e", "b", "f#", "c#", "g#", "d#",
];
const FLATS: [&str; 12] = [
    "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
];

// base sharp notes
// A, A#, B, C, C#, D, D#, E, F, F#, G, G#
const BASE_SHARP_NOTES: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

// base flat notes
// A, Bb, B, C, Db, D, Eb, E, F, Gb, G, Ab
const BASE_FLAT_NOTES: [&str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];
pub struct Scale {
    notes: Vec<String>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        // unimplemented!(
        //     "Construct a new scale with tonic {} and intervals {}",
        //     tonic,
        //     intervals
        // )
        let origin = Scale::chromatic(tonic)?; // throw error if has
        let mut notes = Vec::new();
        let mut i = 0;
        for interval in intervals.chars() {
            notes.push(origin.notes[i].clone());
            match interval {
                'm' => i += 1,
                'M' => i += 2,
                'A' => i += 3,
                _ => return Err(Error),
            }
        }
        notes.push(notes[0].clone());
        Ok(Scale { notes })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        // unimplemented!("Construct a new chromatic scale with tonic {}", tonic)

        // if tonic includes SHARPS ,then notes equals to BASE_SHARP_NOTES
        // else if tonic includes FLATS ,then notes equals to BASE_FLAT_NOT

        let notes;

        if SHARPS.contains(&tonic) {
            notes = BASE_SHARP_NOTES.to_vec();
        } else if FLATS.contains(&tonic) {
            notes = BASE_FLAT_NOTES.to_vec();
        } else {
            return Err(Error);
        }

        let ans = notes
            .iter()
            .cycle()
            .skip_while(|x| x.to_lowercase() != tonic.to_lowercase())
            .take(13)
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        Ok(Self { notes: ans })
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.notes.clone()
    }
}
