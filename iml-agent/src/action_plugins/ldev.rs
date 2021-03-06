// Copyright (c) 2020 DDN. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{agent_error::ImlAgentError, env};
use iml_wire_types::LdevEntry;
use std::collections::BTreeSet;
use tokio::{
    fs::{metadata, read_to_string, File},
    io::AsyncWriteExt,
};

async fn write_to_file(content: String) -> Result<(), ImlAgentError> {
    let ldev_path = env::get_ldev_conf();
    let mut file = File::create(ldev_path).await?;
    file.write_all(content.as_bytes()).await?;

    Ok(())
}

async fn read_ldev_config() -> Result<String, ImlAgentError> {
    let ldev_path = env::get_ldev_conf();

    match metadata(&ldev_path).await {
        Ok(_) => Ok(read_to_string(&ldev_path).await?),
        Err(_) => Ok("".into()),
    }
}

fn parse_entries(ldev_config: String) -> BTreeSet<LdevEntry> {
    ldev_config.lines().map(LdevEntry::from).collect()
}

fn convert(entries: &[LdevEntry]) -> String {
    entries
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

pub async fn create(entries: Vec<LdevEntry>) -> Result<(), ImlAgentError> {
    if !entries.is_empty() {
        let ldev_config = read_ldev_config().await?;
        let existing_entries = parse_entries(ldev_config);
        let entries_set = entries.iter().cloned().collect::<BTreeSet<LdevEntry>>();
        if existing_entries != entries_set {
            let data = convert(&entries);
            write_to_file(data).await?;
        }

        Ok(())
    } else {
        Err(ImlAgentError::LdevEntriesError(
            "The ldev entries must not be empty.".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iml_wire_types::FsType;

    #[test]
    fn test_create() -> Result<(), ImlAgentError> {
        let entries = vec![
            LdevEntry {
                primary: "mds1".into(),
                failover: Some("mds2".into()),
                label: "MGS".into(),
                device: "zfs:mdt0/mdt0".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "mds1".into(),
                failover: Some("mds2".into()),
                label: "zfsmo-MDT0000".into(),
                device: "zfs:mdt0/mdt0".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "mds2".into(),
                failover: Some("mds1".into()),
                label: "zfsmo-MDT0001".into(),
                device: "zfs:mdt1/mdt1".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss1".into(),
                failover: Some("oss2".into()),
                label: "zfsmo-OST0000".into(),
                device: "zfs:ost0/ost0".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss1".into(),
                failover: Some("oss2".into()),
                label: "zfsmo-OST0001".into(),
                device: "zfs:ost1/ost1".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss1".into(),
                failover: Some("oss2".into()),
                label: "zfsmo-OST0002".into(),
                device: "zfs:ost2/ost2".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss1".into(),
                failover: Some("oss2".into()),
                label: "zfsmo-OST0003".into(),
                device: "zfs:ost3/ost3".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss1".into(),
                failover: Some("oss2".into()),
                label: "zfsmo-OST0004".into(),
                device: "zfs:ost4/ost4".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss1".into(),
                failover: Some("oss2".into()),
                label: "zfsmo-OST0005".into(),
                device: "zfs:ost5/ost5".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss1".into(),
                failover: Some("oss2".into()),
                label: "zfsmo-OST0006".into(),
                device: "zfs:ost6/ost6".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss1".into(),
                failover: Some("oss2".into()),
                label: "zfsmo-OST0007".into(),
                device: "zfs:ost7/ost7".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss1".into(),
                failover: Some("oss2".into()),
                label: "zfsmo-OST0008".into(),
                device: "zfs:ost8/ost8".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss1".into(),
                failover: Some("oss2".into()),
                label: "zfsmo-OST0009".into(),
                device: "zfs:ost9/ost9".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss2".into(),
                failover: Some("oss1".into()),
                label: "zfsmo-OST000a".into(),
                device: "zfs:ost10/ost10".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss2".into(),
                failover: Some("oss1".into()),
                label: "zfsmo-OST000b".into(),
                device: "zfs:ost11/ost11".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss2".into(),
                failover: Some("oss1".into()),
                label: "zfsmo-OST000c".into(),
                device: "zfs:ost12/ost12".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss2".into(),
                failover: Some("oss1".into()),
                label: "zfsmo-OST000d".into(),
                device: "zfs:ost13/ost13".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss2".into(),
                failover: Some("oss1".into()),
                label: "zfsmo-OST000e".into(),
                device: "zfs:ost14/ost14".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss2".into(),
                failover: Some("oss1".into()),
                label: "zfsmo-OST000f".into(),
                device: "zfs:ost15/ost15".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss2".into(),
                failover: Some("oss1".into()),
                label: "zfsmo-OST0010".into(),
                device: "zfs:ost16/ost16".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss2".into(),
                failover: Some("oss1".into()),
                label: "zfsmo-OST00011".into(),
                device: "zfs:ost17/ost17".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss2".into(),
                failover: Some("oss1".into()),
                label: "zfsmo-OST00012".into(),
                device: "zfs:ost18/ost18".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "oss2".into(),
                failover: Some("oss1".into()),
                label: "zfsmo-OST00013".into(),
                device: "zfs:ost19/ost19".into(),
                fs_type: Some(FsType::Zfs),
            },
        ]
        .into_iter()
        .collect::<Vec<LdevEntry>>();

        let data = convert(&entries);
        insta::assert_snapshot!(data);

        Ok(())
    }

    #[test]
    fn test_config_without_ha() -> Result<(), ImlAgentError> {
        let entries = vec![
            LdevEntry {
                primary: "mds1".into(),
                failover: None,
                label: "MGS".into(),
                device: "zfs:mdt0/mdt0".into(),
                fs_type: Some(FsType::Zfs),
            },
            LdevEntry {
                primary: "mds1".into(),
                failover: Some("mds2".into()),
                label: "zfsmo-MDT0000".into(),
                device: "zfs:mdt0/mdt0".into(),
                fs_type: Some(FsType::Zfs),
            },
        ]
        .into_iter()
        .collect::<Vec<LdevEntry>>();

        let data = convert(&entries);
        insta::assert_snapshot!(data);

        Ok(())
    }
}
