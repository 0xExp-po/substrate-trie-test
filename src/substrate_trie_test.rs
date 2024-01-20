#![allow(unused_imports)]

use hex_literal::hex;
use sp_core::{Blake2Hasher, H256};
use sp_trie::{MemoryDB, LayoutV1, trie_types::TrieDBMutBuilderV1, TrieMut, TrieHash, TrieDBMut};

#[test]
pub fn root() {
    let mut db = MemoryDB::new(&[0u8]);
    let mut root: TrieHash<LayoutV1<Blake2Hasher>> = Default::default();

    let mut t: TrieDBMut<'_, LayoutV1<Blake2Hasher>> = TrieDBMutBuilderV1::new(&mut db, &mut root).build();

    let entries: Vec<(&[u8], &[u8])> = vec![
		// "alfa" is at a hash-referenced leaf node.
		(b"alfa", &[0; 40]),
		// "bravo" is at an inline leaf node.
		(b"bravo", b"bravo"),
		// "do" is at a hash-referenced branch node.
		(b"do", b"verb"),
		// "dog" is at a hash-referenced branch node.
		(b"dog", &[0; 40]),
		// "doge" is at a hash-referenced leaf node.
		(b"doge", &[0; 40]),
		// extension node "o" (plus nibble) to next branch.
		(b"horse", b"stallion"),
		(b"house", b"building"),
	];

    for entry in entries {
       let _ = t.insert(entry.0, entry.1).expect("inserted");
    }

    t.commit();

	let root = &H256(hex!["f465577d5a13b1a48ea7245c559ab78175eb509b00829cf5db09d61ea795b74a"]);
    let actual: &TrieHash<LayoutV1<Blake2Hasher>> = t.root();

    println!("ROOT {:?}", actual);

    assert_eq!(
        actual,
        root
    );
}