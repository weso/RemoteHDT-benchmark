use rand::thread_rng;
use rand::Rng;
use remote_hdt::storage::layout::Layout;
use remote_hdt::storage::ops::Ops;
use remote_hdt::storage::ops::OpsResult;
use remote_hdt::storage::params::Backend;
use remote_hdt::storage::params::Serialization;
use remote_hdt::storage::Storage;
use std::time::Duration;
use std::time::Instant;

use crate::utils::write_csv;

pub struct Line<'a> {
    pub layout: &'a str,
    pub serialization: &'a str,
    pub file_name: &'a str,
    pub subject: Duration,
    pub predicate: Duration,
    pub object: Duration,
}

impl<'a> Line<'a> {
    fn new(
        layout: &'a str,
        serialization: &'a str,
        file_name: &'a str,
        subject: Duration,
        predicate: Duration,
        object: Duration,
    ) -> Self {
        Self {
            layout,
            serialization,
            file_name,
            subject,
            predicate,
            object,
        }
    }
}

pub struct BenchmarkExecutor {
    iterations: u8,
}

impl BenchmarkExecutor {
    pub fn new(iterations: u8) -> Self {
        Self { iterations }
    }

    pub fn execute<C>(
        &self,
        layout: impl Layout<C> + 'static,
        serialization: Serialization,
        backend: &Backend,
    ) {
        let mut times = Vec::<Line>::new();

        let binding = layout.name();
        let layout_name = binding.as_str();

        let serialization_name = match serialization {
            Serialization::Zarr => "zarr",
            Serialization::Sparse => "sparse",
        };

        let mut storage = Storage::new(layout, serialization);
        let (arr, file_name) = match backend {
            Backend::FileSystem(path) => (storage.load(Backend::FileSystem(path)).unwrap(), path),
            Backend::HTTP(url) => (storage.load(Backend::HTTP(url)).unwrap(), url),
        };
        let dictionary = &arr.get_dictionary();

        let mut rng = thread_rng();

        let subject = String::from_utf8(
            dictionary
                .subjects()
                .decoder()
                .run(rng.gen_range(0..dictionary.subjects_size())),
        )
        .unwrap();

        let predicate = String::from_utf8(
            dictionary
                .predicates()
                .decoder()
                .run(rng.gen_range(0..dictionary.predicates_size())),
        )
        .unwrap();

        let object = String::from_utf8(
            dictionary
                .objects()
                .decoder()
                .run(rng.gen_range(0..dictionary.objects_size())),
        )
        .unwrap();

        // Do the operation twice before executing to avoid cold-start
        for _ in 0..3 {
            let _ = self.query(|| arr.get_subject(&subject));
            let _ = self.query(|| arr.get_predicate(&predicate));
            let _ = self.query(|| arr.get_object(&object));
        }

        // The actual benchmark begins
        for _ in 0..self.iterations {
            let subject_duration = self.query(|| arr.get_subject(&subject));
            let predicate_duration = self.query(|| arr.get_predicate(&predicate));
            let object_duration = self.query(|| arr.get_object(&object));

            times.push(Line::new(
                layout_name,
                serialization_name,
                file_name,
                subject_duration,
                predicate_duration,
                object_duration,
            ));
        }

        write_csv(times);
    }

    fn query(&self, query: impl Fn() -> OpsResult) -> Duration {
        let before = Instant::now();
        let _ = query();
        before.elapsed()
    }
}
