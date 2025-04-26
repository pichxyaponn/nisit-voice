-- Your SQL goes here
CREATE TABLE reports (
    id SERIAL PRIMARY KEY,
    "title" VARCHAR(255) NOT NULL,
    "description" TEXT,
    "status" VARCHAR(100) NOT NULL,
    nisit_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    resolved_at TIMESTAMP,
    removed_at TIMESTAMP
);

CREATE TABLE staff (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE nisits (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE report_staff_junction (
    report_id INTEGER NOT NULL,
    staff_id INTEGER NOT NULL,
    assigned_at TIMESTAMP NOT NULL DEFAULT now(), -- เวลาที่ staff คนนี้ถูกเพิ่มเข้ามาในเรื่องนี้
    PRIMARY KEY (report_id, staff_id)
);

ALTER TABLE reports
ADD CONSTRAINT fk_nisit FOREIGN KEY (nisit_id) REFERENCES nisits(id);

-- เชื่อม report_staff_junction กับ reports
ALTER TABLE report_staff_junction
ADD CONSTRAINT fk_report FOREIGN KEY (report_id) REFERENCES reports(id);

-- เชื่อม report_staff_junction กับ staff
ALTER TABLE report_staff_junction
ADD CONSTRAINT fk_staff FOREIGN KEY (staff_id) REFERENCES staff(id);
