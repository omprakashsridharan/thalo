SELECT MAX("sequence") FROM "event" WHERE "aggregate_type" = $1 AND "aggregate_id" = $2;