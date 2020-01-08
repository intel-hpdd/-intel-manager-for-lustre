use crate::{
    components::{alert_indicator, font_awesome, paging, Placement},
    generated::css_classes::C,
    Route,
};
use iml_wire_types::{
    db::{Id, OstPoolRecord, VolumeNodeRecord},
    warp_drive::{Cache, RecordId},
    Filesystem, Host, Label, Target, TargetConfParam, TargetKind,
};
use seed::{prelude::*, *};
use std::{
    collections::{BTreeSet, HashMap},
    iter::{once, FromIterator},
    ops::{Deref, DerefMut},
};

fn sort_by_label(xs: &mut Vec<impl Label>) {
    xs.sort_by(|a, b| natord::compare(a.label(), b.label()));
}

pub fn slice_page<'a>(paging: &paging::Model, xs: &'a BTreeSet<u32>) -> impl Iterator<Item = &'a u32> {
    xs.iter().skip(paging.offset()).take(paging.end())
}

fn sorted_cache<'a>(x: &'a im::HashMap<u32, impl Label + Id>) -> impl Iterator<Item = u32> + 'a {
    let mut xs: Vec<_> = x.values().collect();

    xs.sort_by(|a, b| natord::compare(a.label(), b.label()));

    xs.into_iter().map(|x| x.id())
}

fn get_volume_nodes_by_host_id(xs: &im::HashMap<u32, VolumeNodeRecord>, host_id: u32) -> Vec<&VolumeNodeRecord> {
    xs.values().filter(|v| v.host_id == host_id).collect()
}

fn get_ost_pools_by_fs_id(xs: &im::HashMap<u32, OstPoolRecord>, fs_id: u32) -> Vec<&OstPoolRecord> {
    xs.values().filter(|v| v.filesystem_id == fs_id).collect()
}

fn get_targets_by_parent_resource(
    cache: &Cache,
    parent_resource_id: RecordId,
    kind: TargetKind,
) -> Vec<&Target<TargetConfParam>> {
    match parent_resource_id {
        RecordId::OstPool(x) => get_targets_by_pool_id(cache, x),
        RecordId::Filesystem(x) => get_targets_by_fs_id(&cache.target, x, kind),
        _ => vec![],
    }
}

fn get_targets_by_pool_id(cache: &Cache, ostpool_id: u32) -> Vec<&Target<TargetConfParam>> {
    let target_ids: Vec<_> = cache
        .ost_pool_osts
        .values()
        .filter(|x| x.ostpool_id == ostpool_id)
        .map(|x| x.managedost_id)
        .collect();

    cache.target.values().filter(|x| target_ids.contains(&x.id)).collect()
}

fn get_targets_by_fs_id(
    xs: &im::HashMap<u32, Target<TargetConfParam>>,
    fs_id: u32,
    kind: TargetKind,
) -> Vec<&Target<TargetConfParam>> {
    xs.values()
        .filter(|x| match kind {
            TargetKind::Mgt => {
                x.kind == TargetKind::Mgt
                    && x.filesystems
                        .as_ref()
                        .and_then(|ys| ys.iter().find(|y| y.id == fs_id))
                        .is_some()
            }
            TargetKind::Mdt => x.kind == TargetKind::Mdt && x.filesystem_id == Some(fs_id),
            TargetKind::Ost => x.kind == TargetKind::Ost && x.filesystem_id == Some(fs_id),
        })
        .collect()
}

fn get_target_fs_ids(x: &Target<TargetConfParam>) -> Vec<u32> {
    match x.kind {
        TargetKind::Mgt => x.filesystems.iter().flatten().map(|x| x.id).collect(),
        TargetKind::Mdt | TargetKind::Ost => x.filesystem_id.map(|x| vec![x]).unwrap_or_default(),
    }
}

// Model

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum Step {
    HostCollection,
    Host(u32),
    VolumeCollection,
    FsCollection,
    Fs(u32),
    MgtCollection,
    MdtCollection,
    OstPoolCollection,
    OstPool(u32),
    OstCollection,
}

impl From<TargetKind> for Step {
    fn from(target_kind: TargetKind) -> Self {
        match target_kind {
            TargetKind::Mgt => Step::MgtCollection,
            TargetKind::Mdt => Step::MdtCollection,
            TargetKind::Ost => Step::OstCollection,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Hash, Clone)]
pub struct Address(BTreeSet<Step>);

impl Address {
    fn new(path: impl IntoIterator<Item = Step>) -> Self {
        Address(BTreeSet::from_iter(path))
    }
    fn extend(&self, step: impl Into<Step>) -> Address {
        Address::new(self.iter().copied().chain(once(step.into())))
    }
    fn as_vec(&self) -> Vec<Step> {
        self.iter().copied().collect()
    }
}

impl Deref for Address {
    type Target = BTreeSet<Step>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Step>> for Address {
    fn from(xs: Vec<Step>) -> Self {
        Address::new(xs)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TreeNode {
    open: bool,
    items: BTreeSet<u32>,
    paging: paging::Model,
}

impl TreeNode {
    fn from_items(xs: impl IntoIterator<Item = u32>) -> Self {
        let items = BTreeSet::from_iter(xs);

        TreeNode {
            open: false,
            paging: paging::Model::new(items.len()),
            items,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Model(HashMap<Address, TreeNode>);

impl Deref for Model {
    type Target = HashMap<Address, TreeNode>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Model {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Model {
    fn reset(&mut self) {
        self.0 = HashMap::new();
    }
    fn remove_item(&mut self, addr: &Address, id: u32) {
        if let Some(tree_node) = self.get_mut(addr) {
            tree_node.items.remove(&id);
            tree_node.paging.total -= 1;
        }
    }
}

// Update

fn add_item(record_id: RecordId, cache: &Cache, model: &mut Model, orders: &mut impl Orders<Msg>) -> Option<()> {
    match record_id {
        RecordId::Host(id) => {
            let addr: Address = vec![Step::HostCollection].into();

            let tree_node = model.get_mut(&addr)?;

            tree_node.items = sorted_cache(&cache.host).collect();
            tree_node.paging.total = tree_node.items.len();

            orders.send_msg(Msg::AddEmptyNode(addr.extend(Step::Host(id))));
        }
        RecordId::Filesystem(id) => {
            let addr: Address = vec![Step::FsCollection].into();

            let tree_node = model.get_mut(&addr)?;

            tree_node.items = sorted_cache(&cache.filesystem).collect();
            tree_node.paging.total = tree_node.items.len();

            orders.send_msg(Msg::AddEmptyNode(addr.extend(Step::Fs(id))));
        }
        RecordId::VolumeNode(id) => {
            let vn = cache.volume_node.get(&id)?;

            let tree_node =
                model.get_mut(&vec![Step::HostCollection, Step::Host(vn.host_id), Step::VolumeCollection].into())?;

            tree_node.items.insert(id);

            let mut xs = cache
                .volume_node
                .values()
                .filter(|y| tree_node.items.contains(&y.id))
                .collect();

            sort_by_label(&mut xs);

            tree_node.items = xs.into_iter().map(|x| x.id).collect();
            tree_node.paging.total = tree_node.items.len();
        }
        RecordId::OstPoolOsts(id) => {
            let ost_pool_ost = cache.ost_pool_osts.get(&id)?;
            let ost_pool = cache.ost_pool.get(&ost_pool_ost.ostpool_id)?;

            let tree_node = model.get_mut(
                &vec![
                    Step::FsCollection,
                    Step::Fs(ost_pool.filesystem_id),
                    Step::OstPoolCollection,
                    Step::OstPool(ost_pool.id),
                ]
                .into(),
            )?;

            tree_node.items.insert(id);

            let mut xs = cache
                .ost_pool
                .values()
                .filter(|y| tree_node.items.contains(&y.id))
                .collect();

            sort_by_label(&mut xs);

            tree_node.items = xs.into_iter().map(|x| x.id).collect();
            tree_node.paging.total = tree_node.items.len();
        }
        RecordId::Target(id) => {
            let target = cache.target.get(&id)?;

            let ids = get_target_fs_ids(target);

            let sort_fn = |cache: &Cache, model: &TreeNode| {
                let mut xs = cache.target.values().filter(|y| model.items.contains(&y.id)).collect();

                sort_by_label(&mut xs);

                xs.into_iter().map(|x| x.id).collect()
            };

            for fs_id in ids {
                let base_addr: Address = vec![Step::FsCollection, Step::Fs(fs_id)].into();

                let target_tree_node = model.get_mut(&base_addr.extend(target.kind))?;

                target_tree_node.items.insert(id);

                target_tree_node.items = sort_fn(cache, target_tree_node);
                target_tree_node.paging.total = target_tree_node.items.len();

                let ostcolletion_node =
                    model.get_mut(&base_addr.extend(Step::OstPoolCollection).extend(Step::OstCollection))?;

                ostcolletion_node.items.insert(id);

                ostcolletion_node.items = sort_fn(cache, ostcolletion_node);
                ostcolletion_node.paging.total = ostcolletion_node.items.len();
            }
        }
        _ => {}
    };

    Some(())
}

fn remove_item(record_id: RecordId, cache: &Cache, model: &mut Model, orders: &mut impl Orders<Msg>) -> Option<()> {
    match record_id {
        RecordId::Host(id) => {
            let addr: Address = vec![Step::HostCollection].into();

            model.remove_item(&addr, id);

            orders.send_msg(Msg::RemoveNode(addr.extend(Step::Host(id))));
        }
        RecordId::Filesystem(id) => {
            let addr: Address = vec![Step::FsCollection].into();

            model.remove_item(&addr, id);

            orders.send_msg(Msg::RemoveNode(addr.extend(Step::Fs(id))));
        }
        RecordId::VolumeNode(id) => {
            let vn = cache.volume_node.get(&id)?;

            model.remove_item(
                &vec![Step::HostCollection, Step::Host(vn.host_id), Step::VolumeCollection].into(),
                id,
            );
        }
        RecordId::OstPoolOsts(id) => {
            let ost_pool_ost = cache.ost_pool_osts.get(&id)?;
            let ost_pool = cache.ost_pool.get(&ost_pool_ost.ostpool_id)?;

            let addr: Address = vec![
                Step::FsCollection,
                Step::Fs(ost_pool.filesystem_id),
                Step::OstPoolCollection,
            ]
            .into();

            model.remove_item(&addr, ost_pool.id);

            orders.send_msg(Msg::RemoveNode(addr.extend(Step::OstPool(ost_pool.id))));
        }
        RecordId::Target(id) => {
            let target = cache.target.get(&id)?;

            let ids = get_target_fs_ids(target);

            for fs_id in ids {
                let addr: Address = vec![Step::FsCollection, Step::Fs(fs_id)].into();

                model.remove_item(&addr.extend(target.kind), id);

                model.remove_item(&addr.extend(Step::OstPoolCollection).extend(Step::OstCollection), id);
            }
        }
        _ => {}
    };

    Some(())
}

#[derive(Clone)]
pub enum Msg {
    Add(RecordId),
    Remove(RecordId),
    Reset,
    Toggle(Address, bool),
    AddEmptyNode(Address),
    RemoveNode(Address),
    Page(Address, paging::Msg),
}

pub fn update(cache: &Cache, msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Reset => {
            model.reset();

            // Add hosts
            model.insert(
                vec![Step::HostCollection].into(),
                TreeNode::from_items(sorted_cache(&cache.host)),
            );

            // Add fs
            model.insert(
                vec![Step::FsCollection].into(),
                TreeNode::from_items(sorted_cache(&cache.filesystem)),
            );
        }
        Msg::Add(id) => {
            add_item(id, cache, model, orders);
        }
        Msg::Remove(id) => {
            remove_item(id, cache, model, orders);
        }
        Msg::Toggle(address, open) => {
            let tree_node = match model.get_mut(&address) {
                Some(x) => x,
                None => return,
            };

            tree_node.open = open;

            let paging: Vec<_> = tree_node.items.iter().copied().collect();

            match address.as_vec().as_slice() {
                [Step::HostCollection] => {
                    paging.into_iter().for_each(|x| {
                        model
                            .entry(address.extend(Step::Host(x)))
                            .or_insert_with(TreeNode::default);
                    });
                }
                [Step::HostCollection, Step::Host(id)] => {
                    model.entry(address.extend(Step::VolumeCollection)).or_insert_with(|| {
                        let mut xs = get_volume_nodes_by_host_id(&cache.volume_node, *id);

                        sort_by_label(&mut xs);

                        TreeNode::from_items(xs.into_iter().map(|x| x.id))
                    });
                }
                [Step::FsCollection] => {
                    paging.into_iter().for_each(|x| {
                        model
                            .entry(address.extend(Step::Fs(x)))
                            .or_insert_with(TreeNode::default);
                    });
                }
                [Step::FsCollection, Step::Fs(id)] => {
                    model.entry(address.extend(Step::MgtCollection)).or_insert_with(|| {
                        let mut xs = get_targets_by_fs_id(&cache.target, *id, TargetKind::Mgt);

                        sort_by_label(&mut xs);

                        TreeNode::from_items(xs.into_iter().map(|x| x.id))
                    });

                    model.entry(address.extend(Step::MdtCollection)).or_insert_with(|| {
                        let mut xs = get_targets_by_fs_id(&cache.target, *id, TargetKind::Mdt);

                        sort_by_label(&mut xs);

                        TreeNode::from_items(xs.into_iter().map(|x| x.id))
                    });

                    model.entry(address.extend(Step::OstCollection)).or_insert_with(|| {
                        let mut xs = get_targets_by_fs_id(&cache.target, *id, TargetKind::Ost);

                        sort_by_label(&mut xs);

                        TreeNode::from_items(xs.into_iter().map(|x| x.id))
                    });

                    model.entry(address.extend(Step::OstPoolCollection)).or_insert_with(|| {
                        let mut xs = get_ost_pools_by_fs_id(&cache.ost_pool, *id);

                        sort_by_label(&mut xs);

                        TreeNode::from_items(xs.into_iter().map(|x| x.id))
                    });
                }
                [Step::FsCollection, Step::Fs(_), Step::OstPoolCollection] => {
                    paging.into_iter().for_each(|x| {
                        model
                            .entry(address.extend(Step::OstPool(x)))
                            .or_insert_with(TreeNode::default);
                    });
                }
                [Step::FsCollection, Step::Fs(_), Step::OstPoolCollection, Step::OstPool(pool_id)] => {
                    model.entry(address.extend(Step::OstCollection)).or_insert_with(|| {
                        let mut xs =
                            get_targets_by_parent_resource(cache, RecordId::OstPool(*pool_id), TargetKind::Ost);

                        sort_by_label(&mut xs);

                        TreeNode::from_items(xs.into_iter().map(|x| x.id))
                    });
                }
                _ => {}
            };
        }
        Msg::AddEmptyNode(addr) => {
            model.insert(addr, TreeNode::default());
        }
        Msg::RemoveNode(id) => {
            model.remove(&id);
        }
        Msg::Page(id, msg) => {
            if let Some(x) = model.get_mut(&id) {
                paging::update(msg, &mut x.paging)
            }
        }
    }
}

// View
fn toggle_view(address: Address, is_open: bool) -> Node<Msg> {
    let mut toggle = font_awesome(
        class![
            C.select_none,
            C.hover__text_gray_300,
            C.cursor_pointer,
            C.w_5,
            C.h_4,
            C.inline,
            C.mr_1,
        ],
        "chevron-right",
    );

    toggle.add_listener(mouse_ev(Ev::Click, move |_| Msg::Toggle(address, !is_open)));

    if is_open {
        toggle.add_style(St::Transform, "rotate(90deg)");
    }

    toggle
}

fn item_view(icon: &str, label: &str, route: Route) -> Node<Msg> {
    a![
        class![C.hover__underline, C.hover__text_gray_300, C.mr_1],
        attrs! {
            At::Href => route.to_href()
        },
        font_awesome(class![C.w_5, C.h_4, C.inline, C.mr_1], icon),
        label
    ]
}

fn tree_host_item_view(cache: &Cache, model: &Model, host: &Host) -> Option<Node<Msg>> {
    let address = Address::new(vec![Step::HostCollection, Step::Host(host.id)]);

    let tree_node = model.get(&address)?;

    Some(li![
        class![C.py_1],
        toggle_view(address.clone(), tree_node.open),
        item_view("server", &host.label, Route::ServerDetail(host.id.into())),
        alert_indicator(&cache.active_alert, &host.resource_uri, true, Placement::Bottom),
        if tree_node.open {
            tree_volume_collection_view(cache, model, &address)
        } else {
            empty![]
        }
    ])
}

fn tree_pool_item_view(cache: &Cache, model: &Model, address: &Address, pool: &OstPoolRecord) -> Option<Node<Msg>> {
    let address = address.extend(Step::OstPool(pool.id));

    let tree_node = model.get(&address)?;

    Some(li![
        class![C.py_1],
        toggle_view(address.clone(), tree_node.open),
        item_view("swimming-pool", pool.label(), Route::OstPoolDetail(pool.id.into())),
        if tree_node.open {
            tree_target_collection_view(cache, model, &address, TargetKind::Ost)
        } else {
            empty![]
        }
    ])
}

fn tree_fs_item_view(cache: &Cache, model: &Model, fs: &Filesystem) -> Option<Node<Msg>> {
    let address = Address::new(vec![Step::FsCollection, Step::Fs(fs.id)]);

    let tree_node = model.get(&address)?;

    Some(li![
        class![C.py_1],
        toggle_view(address.clone(), tree_node.open),
        item_view("server", &fs.label, Route::FilesystemDetail(fs.id.into())),
        alert_indicator(&cache.active_alert, &fs.resource_uri, true, Placement::Bottom),
        if tree_node.open {
            vec![
                tree_target_collection_view(cache, model, &address, TargetKind::Mgt),
                tree_target_collection_view(cache, model, &address, TargetKind::Mdt),
                tree_target_collection_view(cache, model, &address, TargetKind::Ost),
                tree_pools_collection_view(cache, model, &address),
            ]
        } else {
            vec![]
        }
    ])
}

fn tree_collection_view(
    model: &Model,
    address: Address,
    item: impl FnOnce(&TreeNode) -> Node<Msg>,
    on_open: impl FnOnce(&TreeNode) -> Vec<Node<Msg>>,
) -> Option<Node<Msg>> {
    let x = model.get(&address)?;

    let el = ul![
        class![C.px_6, C.mt_2],
        toggle_view(address.clone(), x.open),
        item(x),
        if x.open {
            ul![
                class![C.px_6, C.mt_2],
                on_open(x),
                li![
                    class![C.py_1],
                    paging::next_prev_view(&x.paging).map_msg(move |msg| { Msg::Page(address, msg) })
                ]
            ]
        } else {
            empty![]
        }
    ];

    Some(el)
}

fn tree_fs_collection_view(cache: &Cache, model: &Model) -> Node<Msg> {
    tree_collection_view(
        model,
        Address::new(vec![Step::FsCollection]),
        |x| {
            item_view(
                "folder",
                &format!("Filesystems ({})", x.paging.total()),
                Route::Filesystem,
            )
        },
        |x| {
            slice_page(&x.paging, &x.items)
                .filter_map(|x| cache.filesystem.get(x))
                .filter_map(|x| tree_fs_item_view(cache, model, x))
                .collect()
        },
    )
    .unwrap_or(empty![])
}

fn tree_host_collection_view(cache: &Cache, model: &Model) -> Node<Msg> {
    tree_collection_view(
        model,
        Address::new(vec![Step::HostCollection]),
        |x| item_view("folder", &format!("Servers ({})", x.paging.total()), Route::Server),
        |x| {
            slice_page(&x.paging, &x.items)
                .filter_map(|x| cache.host.get(x))
                .filter_map(|x| tree_host_item_view(cache, model, x))
                .collect()
        },
    )
    .unwrap_or(empty![])
}

fn tree_pools_collection_view(cache: &Cache, model: &Model, parent_address: &Address) -> Node<Msg> {
    let addr = parent_address.extend(Step::OstPoolCollection);

    tree_collection_view(
        model,
        addr.clone(),
        |x| item_view("folder", &format!("OST Pools ({})", x.paging.total()), Route::OstPool),
        |x| {
            slice_page(&x.paging, &x.items)
                .filter_map(|x| cache.ost_pool.get(x))
                .filter_map(|x| tree_pool_item_view(cache, model, &addr, x))
                .collect()
        },
    )
    .unwrap_or(empty![])
}

fn tree_volume_collection_view(cache: &Cache, model: &Model, parent_address: &Address) -> Node<Msg> {
    tree_collection_view(
        model,
        parent_address.extend(Step::VolumeCollection),
        |x| item_view("folder", &format!("Volumes ({})", x.paging.total()), Route::Volume),
        |x| {
            slice_page(&x.paging, &x.items)
                .filter_map(|x| cache.volume_node.get(x))
                .map(|x| {
                    let v = cache.volume.values().find(|v| v.id == x.volume_id).unwrap();

                    let size = match v.size {
                        Some(x) => format!(" ({})", number_formatter::format_bytes(x as f64, None)),
                        None => "".into(),
                    };

                    li![
                        class![C.py_1],
                        item_view(
                            "hdd",
                            &format!("{}{}", x.label(), size),
                            Route::VolumeDetail(v.id.into())
                        ),
                    ]
                })
                .collect()
        },
    )
    .unwrap_or(empty![])
}

fn tree_target_collection_view(cache: &Cache, model: &Model, parent_address: &Address, kind: TargetKind) -> Node<Msg> {
    let label = match kind {
        TargetKind::Mgt => "MGTs",
        TargetKind::Mdt => "MDTs",
        TargetKind::Ost => "OSTs",
    };

    tree_collection_view(
        model,
        parent_address.extend(kind),
        |x| item_view("folder", &format!("{} ({})", label, x.paging.total()), Route::Target),
        |x| {
            slice_page(&x.paging, &x.items)
                .filter_map(|x| cache.target.get(x))
                .map(|x| {
                    li![
                        class![C.py_1],
                        item_view("bullseye", x.label(), Route::TargetDetail(x.id.into())),
                        alert_indicator(&cache.active_alert, &x.resource_uri, true, Placement::Bottom),
                    ]
                })
                .collect()
        },
    )
    .unwrap_or(empty![])
}

pub fn view(cache: &Cache, model: &Model) -> Node<Msg> {
    div![
        class![C.p_5, C.text_gray_500],
        tree_host_collection_view(cache, model),
        tree_fs_collection_view(cache, model),
    ]
}

#[cfg(test)]
mod tests {
    use super::{update, Address, Model, Msg, Step};
    use crate::test_utils::{create_app_simple, fixtures};
    use seed::virtual_dom::Node;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    fn create_app() -> seed::App<Msg, Model, Node<Msg>> {
        create_app_simple(
            |msg, model, orders| {
                update(&fixtures::get_cache(), msg, model, orders);
            },
            |_| seed::empty(),
        )
    }

    #[wasm_bindgen_test]
    fn test_default_model() {
        let app = create_app();

        let model = app.data.model.borrow();

        assert_eq!(model.as_ref(), Some(&Model::default()));
    }

    #[wasm_bindgen_test]
    fn test_model_reset() {
        let app = create_app();

        app.update(Msg::Reset);

        let model = app.data.model.borrow();

        let expected = vec![
            Address::new(vec![Step::HostCollection]),
            Address::new(vec![Step::FsCollection]),
        ];

        let actual: Vec<_> = model.as_ref().unwrap().keys().cloned().collect();

        assert_eq!(actual, expected);
    }
}