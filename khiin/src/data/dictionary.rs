use anyhow::Result;

use crate::config::engine_cfg::InputType;

use super::database::Database;
use super::segmenter::Segmenter;
use super::trie::Trie;

pub struct Dictionary {
    word_trie: Trie,
    segmenter: Segmenter,
}

impl Dictionary {
    pub fn new(db: &Database, input_type: InputType) -> Result<Self> {
        let inputs = db.all_words_by_freq(input_type)?;

        let word_trie = Trie::new(&inputs)?;
        let segmenter = Segmenter::new(inputs)?;

        Ok(Self {
            word_trie,
            segmenter,
        })
    }

    pub fn find_words_by_prefix(&self, query: &str) -> Vec<u32> {
        self.word_trie.find_words_by_prefix(query)
    }

    pub fn segment(&self, query: &str) -> Result<Vec<String>> {
        self.segmenter.split(query)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::get_db;

    use super::*;

    fn setup() -> Dictionary {
        let db = get_db();
        Dictionary::new(&db, InputType::Numeric).unwrap()
    }

    #[test]
    fn it_loads() {
        let db = get_db();
        let dict = Dictionary::new(&db, InputType::Numeric);
        assert!(dict.is_ok());
    }

    #[test]
    fn it_finds_words_by_prefix() {
        let dict = setup();
        let ids = dict.find_words_by_prefix("goa");
        assert!(ids.len() > 0);
        let ids = dict.find_words_by_prefix("e");
        assert!(ids.len() > 0);
        let ids = dict.find_words_by_prefix("si");
        assert!(ids.len() > 0);
        let ids2 = dict.find_words_by_prefix("k");
        assert!(ids2.len() > 0);
        let ids = dict.find_words_by_prefix("chh");
        assert!(ids.len() > 0);
        let ids = dict.find_words_by_prefix("a");
        assert!(ids.len() > 0);
    }

    #[test]
    fn it_segments_words() {
        let dict = setup();
        let result =
            dict.segment("lihopengan").expect("Could not segment text");
        assert!(result.len() == 2);
        assert_eq!(result[0].as_str(), "liho");
        assert_eq!(result[1].as_str(), "pengan");
    }

    #[test]
    fn it_segments_long_sentences() {
        let dict = setup();
        let input = "goutuitiunnkinkukasiokthekiongechuliauchitesiaulian\
            kesisimchongbapihlaikoesineiesithekuibinlongsibaksaikapphinn\
            kouchebengbengsitikoesinchinchengsiutiohchintoaethongkhou";
        let result = dict.segment(input).expect("Could not segment text");
        assert!(result.len() > 20);
        assert_eq!(result.join(" ").as_str(),
            "gou tui tiunn kin ku ka siok the kiong e chuliau chite siaulianke \
             si sim chong ba pih lai koe sin e i e sithe kui bin long si \
             baksai kap phinn kou che bengbeng si ti koe sin chincheng siutioh \
             chin toa e thong khou");
        
        // Best time: 1.75 seconds
        // for _ in 0..1000 {
        //     let result = dict.segment(input).expect("Could not segment text");
        //     assert_eq!(result.join(" ").as_str(),
        //     "gou tui tiunn kin ku ka siok the kiong e chuliau chite siaulianke \
        //      si sim chong ba pih lai koe sin e i e sithe kui bin long si \
        //      baksai kap phinn kou che bengbeng si ti koe sin chincheng siutioh \
        //      chin toa e thong khou");
        // }
    }
}
