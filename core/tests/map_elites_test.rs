use darwin_core::genome::graph::GenomeGraph;
use darwin_core::map_elites::MapElitesArchive;
use darwin_core::types::DataType;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn test_map_elites_archive_replacement_and_sampling() {
    let mut archive = MapElitesArchive::new(5, 5);

    assert_eq!(archive.coverage(), (0, 25));
    assert!(archive.best().is_none());

    let mut g1 = GenomeGraph::new(DataType::Float);
    g1.add_input("x", DataType::Float);

    let mut g2 = GenomeGraph::new(DataType::Float);
    g2.add_input("x", DataType::Float);

    // Insert first individual in cell (1, 2) with fitness 0.5
    let replaced1 = archive.add_or_replace(g1.clone(), 0.5, 1, 2);
    assert!(replaced1, "First insert into empty cell should succeed");
    assert_eq!(archive.coverage(), (1, 25));

    // Attempt to insert inferior individual into the same cell (1, 2) with fitness 0.3
    let replaced2 = archive.add_or_replace(g2.clone(), 0.3, 1, 2);
    assert!(!replaced2, "Inferior individual should not replace existing elite in same cell");
    assert_eq!(archive.best().unwrap().1, 0.5);

    // Insert superior individual into the same cell (1, 2) with fitness 0.9
    let replaced3 = archive.add_or_replace(g2.clone(), 0.9, 1, 2);
    assert!(replaced3, "Superior individual must replace existing elite");
    assert_eq!(archive.best().unwrap().1, 0.9);

    // Insert individual in another cell (3, 4) with fitness 0.7
    archive.add_or_replace(g1.clone(), 0.7, 3, 4);
    assert_eq!(archive.coverage(), (2, 25));

    // Verify sampling returns an elite
    let mut rng = StdRng::seed_from_u64(42);
    let sampled = archive.sample_elite(&mut rng);
    assert!(sampled.is_some(), "Sampling should return an elite from an occupied cell");
}
