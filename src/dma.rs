use memflow::{plugins::{ConnectorArgs, args::Args, Inventory, IntoProcessInstanceArcBox}, os::{ModuleInfo, Os, Process}};

pub struct DmaCtx {
    pub process: IntoProcessInstanceArcBox<'static>,
    pub client: ModuleInfo
}

impl DmaCtx {
    pub fn setup(connector: Connector, pcileech_device: String) -> anyhow::Result<DmaCtx> {
        let inventory = Inventory::scan();

        let os = { 
            if connector == Connector::Pcileech {
                let args = Args::new()
                    .insert("device", &pcileech_device);

                let connector_args = ConnectorArgs::new(None, args, None);                

                inventory.builder()
                    .connector(&connector.to_string())
                    .args(connector_args)
                    .os("win32")
                    .build()?
            } else {
                inventory.builder()
                .connector(&connector.to_string())
                .os("win32")
                .build()?
            }
        };

        let mut process = os.into_process_by_name("cs2.exe")?;
        let client = process.module_by_name("client.dll")?;


        Ok(Self {
            process,
            client
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum, Default)]
pub enum Connector {
    #[default]
    Qemu,
    Kvm,
    Pcileech
}

impl ToString for Connector {
    fn to_string(&self) -> String {
        match self {
            Connector::Qemu => String::from("qemu"),
            Connector::Kvm => String::from("kvm"),
            Connector::Pcileech => String::from("pcileech"),
        }
    }
}