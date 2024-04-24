use proguard::ProguardRecord;

pub fn reverse_proguard_record(record: ProguardRecord) -> ProguardRecord {
    match record {
        ProguardRecord::Class {
            original,
            obfuscated,
        } => ProguardRecord::Class {
            obfuscated: original,
            original: obfuscated,
        },

        ProguardRecord::Field {
            ty,
            original,
            obfuscated,
        } => ProguardRecord::Field {
            obfuscated: original,
            original: obfuscated,
            ty,
        },

        ProguardRecord::Method {
            ty,
            original,
            obfuscated,
            arguments,
            original_class,
            line_mapping,
        } => ProguardRecord::Method {
            ty,
            original: obfuscated,
            obfuscated: original,
            arguments,
            original_class,
            line_mapping,
        },

        other => other,
    }
}
