#!/usr/sbin/perl
use List::Util qw(shuffle);
my @items;
while (<STDIN>) {
	push(@items, $_);
}
@items = shuffle(@items);
foreach my $i (@items) {
	print $i;
}
