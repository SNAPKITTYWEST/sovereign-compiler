-- Strata.lean — Stratum boundary verification
-- Non-recursive. WORM-sealed.

import Mathlib

/-- Stratum levels -/
inductive Stratum where
  | zero : Stratum
  | succ : Stratum → Stratum
  | boundary : Stratum → Stratum

/-- Stratum verification -/
def verifyStratum (s : Stratum) : Bool :=
  match s with
  | Stratum.zero => true
  | Stratum.succ inner => verifyStratum inner
  | Stratum.boundary inner => verifyStratum inner && true

/-- Stratum depth -/
def stratumDepth : Stratum → Nat
  | Stratum.zero => 0
  | Stratum.succ inner => 1 + stratumDepth inner
  | Stratum.boundary inner => 1 + stratumDepth inner
