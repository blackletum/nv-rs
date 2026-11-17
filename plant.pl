#!/usr/bin/perl
# plant.pl FILE OLD NEW: replaces the literal text OLD (exactly one occurrence, else dies) by NEW.
# "\n" in OLD/NEW means a line end. Used by sound-mutants.sh to plant one mutant; restoring is
# git checkout of the file.
use strict; use warnings;
my ($file, $old, $new) = @ARGV;
s/\\n/\n/g for ($old, $new);
open(my $fh, '<:raw', $file) or die "open $file: $!";
local $/; my $text = <$fh>; close $fh;
my $crlf = $text =~ /\r\n/;
if ($crlf) { s/\n/\r\n/g for ($old, $new); }
my $count = () = $text =~ /\Q$old\E/g;
die "NOT APPLIED: '$old' found $count times in $file\n" unless $count == 1;
$text =~ s/\Q$old\E/$new/;
open($fh, '>:raw', $file) or die "write $file: $!";
print $fh $text; close $fh;
print "planted in $file\n";
