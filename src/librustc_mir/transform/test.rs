// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rustc::ty::TyCtxt;
use rustc::mir::repr::*;
use rustc::mir::visit::MutVisitor;
use rustc::mir::transform::{MirPass, MirSource, Pass};

struct TestPassVisitor {}

impl TestPassVisitor {
    pub fn new() -> Self {
        TestPassVisitor {}
    }
}

impl<'tcx> MutVisitor<'tcx> for TestPassVisitor {
    fn visit_basic_block_data(&mut self, block: BasicBlock, _: &mut BasicBlockData<'tcx>) {
        debug!("Visit basic block {:?}", block);
    }
}

pub struct TestPass;

impl Pass for TestPass {}

impl<'tcx> MirPass<'tcx> for TestPass {
    fn run_pass<'a>(&mut self, _: TyCtxt<'a, 'tcx, 'tcx>, _: MirSource, mir: &mut Mir<'tcx>) {
        TestPassVisitor::new().visit_mir(mir);
    }
}
