#!/bin/bash
# Planted faults, round 4 (REVIEW-630d, REVIEW-631b): each planted alone, its named test run, the
# fault restored (git checkout) before the next. A fault counts as CAUGHT only when the named test
# itself reports a failure (cargo's "test result: FAILED" with that test FAILED, or test-play.sh's
# "FAIL <check>"), never when the build fails. Usage: bash mutants-r4.sh linux|macos
set -u
system=$1
result=mutants-result.txt
: >"$result"
say() { printf '%s\n' "$*" | tee -a "$result"; }
plant() { perl plant.pl "$@" || { say "NOT PLANTED: $1"; return 1; }; }
restore() { git checkout -- crates viewer distribution; }

# cargo_test ID PKGARGS TEST: runs the one test; CAUGHT when it failed as a test.
cargo_test() {
    id=$1 pkg=$2 name=$3
    # shellcheck disable=SC2086
    cargo test --locked $pkg -- --exact "$name" >out.log 2>&1
    if grep -q "^test $name \.\.\. FAILED" out.log && grep -q "test result: FAILED" out.log; then
        grep -E "panicked at|^test .* FAILED" out.log | head -3
        echo CAUGHT
    elif grep -q "test result: ok. 1 passed" out.log; then
        echo passes
    else
        tail -20 out.log
        echo "NOT RUN (no test result: a build failure is not a catch)"
    fi
}
# play_test ID CHECK [ENV...]: runs test-play.sh under dash (Linux) or /bin/sh (macOS); CAUGHT
# when CHECK is reported FAIL.
play_test() {
    check=$1
    shift
    if [ "$system" = macos ]; then shell=/bin/sh; else shell=dash; fi
    env "$@" SH=$shell sh distribution/unix/test-play.sh >play.log 2>&1
    if grep -qF "FAIL $check" play.log; then
        grep "^FAIL" play.log | head -5
        echo CAUGHT
    elif grep -q "^fails: 0" play.log; then
        echo passes
    else
        grep "^FAIL\|^fails" play.log | head -5
        echo "OTHER (named check did not fail)"
    fi
}
# real_program: the play build of nv-viewer (incremental on top of the job's build).
real_program() {
    cargo build --locked --profile play -p nv-viewer >build.log 2>&1 || { tail -20 build.log; return 1; }
    echo "$PWD/target/play/nv-viewer"
}

PROD=save_out::tests::guard_at_write_time::the_production_check_refuses_a_write_and_a_delete_through_a_swapped_folder
STARTED=save_out::tests::guard_at_write_time::the_game_folder_started_with_is_guarded_unfound
VIEWER=tests::a_game_the_finder_reports_in_another_case_is_taken
CHECKV=tests::the_installation_check_off_windows_checks_the_version_first
STATUS=tests::the_expected_stop_off_windows_has_its_own_status
HOSTLIB="-p host --lib"
VIEWERBIN="-p nv-viewer --bin nv-viewer"

if [ "$system" = linux ]; then
    say "baseline $PROD: $(cargo_test b "$HOSTLIB" $PROD | tail -1)"
    say "baseline $STARTED: $(cargo_test b "$HOSTLIB" $STARTED | tail -1)"
    say "baseline $VIEWER: $(cargo_test b "$VIEWERBIN" $VIEWER | tail -1)"
    say "baseline $CHECKV: $(cargo_test b "$HOSTLIB" $CHECKV | tail -1)"
    say "baseline $STATUS: $(cargo_test b "$HOSTLIB" $STATUS | tail -1)"
    program=$(real_program) && say "baseline test-play.sh with the real program: $(play_test '24 ' NV_RS_REAL_PROGRAM="$program" | tail -1)"

    plant crates/host/src/save_out.rs '.map(|home| game_folders::never_written(&home));' '.map(|_home| Vec::new());' &&
        say "A1 the look's list is empty (prepare): $(cargo_test a1 "$HOSTLIB" $PROD | tail -1)"
    restore
    plant crates/host/src/save_out.rs 'refused_by(guard_list(), path).or_else' 'refused_by(guard_list().map(|_| &[][..]), path).or_else' &&
        say "A2 refused_here ignores the list: $(cargo_test a2 "$HOSTLIB" $PROD | tail -1)"
    restore
    plant crates/host/src/save_out.rs '            warn_refused_mid_run(path, why);\n' '' &&
        say "A3 no warning mid-run: $(cargo_test a3 "$HOSTLIB" $PROD | tail -1)"
    restore
    plant crates/host/src/save_out.rs '        guard_started(data);\n' '        let _ = data;\n' &&
        say "A4 the started game is not guarded: $(cargo_test a4 "$HOSTLIB" $STARTED | tail -1)"
    restore
    plant viewer/src/main.rs 'if let Some(data) = game_folders::data_folder(path) {' 'if let Some(data) = game_folders::data_folder(path).filter(|d| d == path || d.file_name() == Some("Data".as_ref())) {' &&
        say "A5 the program takes Data in its exact case only: $(cargo_test a5 "$VIEWERBIN" $VIEWER | tail -1)"
    restore
    plant crates/host/src/lib.rs '(engine_platform::host_paths::stored_name(&exe), exe.parent())' '(None::<String>, exe.parent())' &&
        say "A6 the exe in its exact case only: $(cargo_test a6 "$VIEWERBIN" $VIEWER | tail -1)"
    restore
    plant crates/host/src/lib.rs '    engine::exe::verify(&exe)?;\n    // Linux and macOS: the game cannot start there yet, so the check says so, after' '    #[cfg(not(windows))]\n    not_provided_here()?;\n    engine::exe::verify(&exe)?;\n    // Linux and macOS: the game cannot start there yet, so the check says so, after' &&
        say "B1 --check stops before the version check: $(cargo_test b1 "$HOSTLIB" $CHECKV | tail -1)"
    restore
    plant crates/host/src/lib.rs 'return NOT_PROVIDED_STATUS;' 'return 1;' &&
        say "B2 the expected stop's status is 1: $(cargo_test b2 "$HOSTLIB" $STATUS | tail -1)"
    restore
    # The program's side of the agreement with play.sh, through the real program (test 24).
    plant viewer/src/main.rs 'if let Some(data) = game_folders::data_folder(path) {' 'if let Some(data) = game_folders::data_folder(path).filter(|d| d == path || d.file_name() == Some("Data".as_ref())) {' &&
        program=$(real_program) && say "B3 the program takes Data in its exact case only, play.sh any: $(play_test '24 the program decides as the stand-in (0): ' NV_RS_REAL_PROGRAM="$program" | tail -1)"
    restore
    plant viewer/src/main.rs '            eprintln!("error: {e}");
            host::end_process(host::error_status(&e));' '            eprintln!("error: {e}");
            host::end_process(1);' &&
        program=$(real_program) && say "B4 the program's expected stop ends with status 1: $(play_test '24 the program decides as the stand-in (0): ' NV_RS_REAL_PROGRAM="$program" | tail -1)"
    restore
fi

# play.sh (both systems)
say "baseline test-play.sh ($system): $(play_test 'none' | tail -1)"
plant distribution/unix/play.sh 'if [ "$status" = "$NOT_PROVIDED_STATUS" ]; then\n    say ""\n    say "$EXPECTED"' 'if false; then\n    say ""\n    say "$EXPECTED"' &&
    say "C1 the expected stop asks for a report: $(play_test '20 no report asked' | tail -1)"
restore
plant distribution/unix/play.sh '    if [ "$status" = "$NOT_PROVIDED_STATUS" ]; then\n        say ""\n        say "Your game' '    if false; then\n        say ""\n        say "Your game' &&
    say "C2 --check's expected stop not said: $(play_test '21 check: said as expected' | tail -1)"
restore
if [ "$system" = macos ]; then
    plant distribution/unix/play.sh 'if [ "$(uname -s)" = Darwin ] && xattr -p' 'if false && xattr -p' &&
        say "C3 the quarantine mark not looked for: $(play_test '22 quarantine: stops' | tail -1)"
    restore
fi
say "=== done ==="
git status --short
