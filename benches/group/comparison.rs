/// See https://bheisler.github.io/criterion.rs/book/getting_started.html to add more benchmarks.
#[macro_use]
extern crate criterion;

use criterion::Criterion;
use crypto::group::{ElemFrom, Group, RSA2048};
use rug::Integer;

fn bench_op<G: Group + ElemFrom<Integer>>() {
  G::op(
    &G::elem(
      Integer::from_str_radix(
        "111066521363124532171649626395987136074128970245601106158251038731392583290069",
        10,
      )
      .unwrap(),
    ),
    &G::elem(
      Integer::from_str_radix(
        "106610920435831899020588753249099054915951032185883121197718271189872278955399",
        10,
      )
      .unwrap(),
    ),
  );
}

fn bench_op_large<G: Group + ElemFrom<Integer>>() {
  G::op(
    &G::elem(Integer::from_str_radix(
      "2172073899553954285893691587818692186975191598984015216589930386158248724081087849265975174\
       9672737203717627738047648700009977053044057502917091973287111671693426065546612150833232954\
       3615367099810550371217642707848747209719337160655740326150736137284544974770721296865388733\
       3057277396369601863707823088589609031265453680152037285312247125429494632830592984498231941\
       6384204134056551840145916685870951507887895129356414704422748714217113880489703934147612551\
       9380825017530552968018297030172607314398711102156189885095451290884843968486448057303474665\
       81515692959313583208325725034506693916571047785061884094866050395109710",
      10,
    )
    .unwrap()),
    &G::elem(Integer::from_str_radix(
      "3172073899553954285893691587818692186975191598984015216589930386158248724081087849265975174\
       9672737203717627738047648700009977053044057502917091973287111671693426065546612150833232954\
       3615367099810550371217642707848747209719337160655740326150736137284544974770721296865388733\
       3057277396369601863707823088589609031265453680152037285312247125429494632830592984498231941\
       6384204134056551840145916685870951507887895129356414704422748714217113880489703934147612551\
       9380825017530552968018297030172607314398711102156189885095451290884843968486448057303474665\
       81515692959313583208325725034506693916571047785061884094866050395109710",
      10,
    )
    .unwrap()),
  );
}

fn bench_exp<G: Group + ElemFrom<u8>>() {
  G::exp(
    &G::elem(2),
    &Integer::from_str_radix(
      "6531513683389606180955725446695124007119189061243576857500117325602044754680002922154438028\
       8474666886816442984548106882909827295319824031764930714696522619672276938781971873901815262\
       4216545626917306691611266738335435709225561930968971212874444236961226918266618788498569915\
       09472508677693535083051665283493383",
      10,
    )
    .unwrap(),
  );
}

fn bench_inv<G: Group + ElemFrom<u8>>() {
  G::inv(&G::elem(2));
}

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("rsa_op", |b| b.iter(bench_op::<RSA2048>));
  c.bench_function("rsa_op_large", |b| b.iter(bench_op_large::<RSA2048>));
  c.bench_function("rsa_exp", |b| b.iter(bench_exp::<RSA2048>));
  c.bench_function("rsa_inv", |b| b.iter(bench_inv::<RSA2048>));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
