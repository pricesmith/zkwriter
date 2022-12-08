BEGIN TRANSACTION;

DROP TABLE [IF EXISTS] zklink;
CREATE TABLE [IF NOT EXISTS] zklink (
    zk_from INTEGER
    zk_to   INTEGER
    FOREIGN KEY (zk_from)
        REFERENCES zknote (id)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    FOREIGN KEY (zk_to)
        REFERENCES zknote (id)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
);

COMMIT;