#!/bin/sh

omt-asset build \
	--content-directory fiiish-content \
	--data-directory fiiish-data \
	--temp-directory temp \
	--paklist fiiish-data/paklist.txt \
	--archive fiiish-data.omar

