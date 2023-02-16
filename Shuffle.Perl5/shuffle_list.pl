#!/usr/bin/perl
use List::Util qw(shuffle);
my @items;
while (<>) { push(@items, $_); }
@items = shuffle(@items);
foreach my $i (@items) { print $i; }
