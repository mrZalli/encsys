extern crate specs;

use encsysman::*;
use enc::*;

type ESMan = EncSysMan<String, String, String>;

struct IntComp(i32);
impl specs::Component for IntComp {
	type Storage = specs::VecStorage<Self>;
}

#[test]
fn ent_from_word() {
	let mut man: ESMan = ESMan::new();
	// the word
	let word = Word::from_collection("word1".to_owned(), vec!["tag1".to_owned()]);
	// register a component
	man.ecs.register_w_comp_id::<IntComp>("comp".to_owned());

	// the word to entity rule function
	let foo = |word: Word<String, String>, builder: &mut MyEntityBuilder<String>| {
		if word.has_tag("tag1".to_owned()) {
			builder.add_comp::<IntComp>("comp".to_owned(), IntComp(1));
		}
	};

	let ent = man.entity_from_word(word, &foo);
	// TODO
}
