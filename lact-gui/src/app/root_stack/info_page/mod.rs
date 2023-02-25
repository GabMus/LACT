mod vulkan_info;

use gtk::prelude::*;
use gtk::*;
use lact_client::schema::{DeviceInfo, DeviceStats};
use vulkan_info::VulkanInfoFrame;

#[derive(Clone)]
pub struct InformationPage {
    pub container: Grid,
    gpu_name_label: Label,
    gpu_manufacturer_label: Label,
    vbios_version_label: Label,
    driver_label: Label,
    vram_size_label: Label,
    link_speed_label: Label,
    vulkan_info_frame: VulkanInfoFrame,
}

impl InformationPage {
    pub fn new() -> Self {
        let container = Grid::new();

        container.set_margin_start(5);
        container.set_margin_end(5);
        container.set_margin_bottom(5);
        container.set_margin_top(5);

        container.set_column_homogeneous(true);

        container.set_row_spacing(7);
        container.set_column_spacing(5);

        // Dummy label to prevent the gpu name label from stealing focus
        let dummy_label = Label::builder()
            .selectable(true)
            .halign(Align::Start)
            .build();
        container.attach(&dummy_label, 0, 0, 1, 1);

        container.attach(
            &{
                let label = Label::new(Some("GPU Model:"));
                label.set_halign(Align::End);
                label
            },
            0,
            0,
            2,
            1,
        );

        let gpu_name_label = value_label();
        container.attach(&gpu_name_label, 2, 0, 3, 1);

        container.attach(
            &{
                let label = Label::new(Some("GPU Manufacturer:"));
                label.set_halign(Align::End);
                label
            },
            0,
            1,
            2,
            1,
        );

        let gpu_manufacturer_label = value_label();
        container.attach(&gpu_manufacturer_label, 2, 1, 3, 1);

        container.attach(
            &{
                let label = Label::new(Some("VBIOS Version:"));
                label.set_halign(Align::End);
                label
            },
            0,
            2,
            2,
            1,
        );

        let vbios_version_label = value_label();
        container.attach(&vbios_version_label, 2, 2, 3, 1);

        container.attach(
            &{
                let label = Label::new(Some("Driver in use:"));
                label.set_halign(Align::End);
                label
            },
            0,
            3,
            2,
            1,
        );

        let driver_label = value_label();
        container.attach(&driver_label, 2, 3, 3, 1);

        container.attach(
            &{
                let label = Label::new(Some("VRAM Size:"));
                label.set_halign(Align::End);
                label
            },
            0,
            4,
            2,
            1,
        );

        let vram_size_label = value_label();
        container.attach(&vram_size_label, 2, 4, 3, 1);

        container.attach(
            &{
                let label = Label::new(Some("Link speed:"));
                label.set_halign(Align::End);
                label
            },
            0,
            5,
            2,
            1,
        );

        let link_speed_label = value_label();
        link_speed_label.set_halign(Align::Start);

        container.attach(&link_speed_label, 2, 5, 3, 1);

        let vulkan_info_frame = VulkanInfoFrame::new();
        container.attach(&vulkan_info_frame.container, 0, 6, 5, 1);

        Self {
            container,
            gpu_name_label,
            gpu_manufacturer_label,
            vbios_version_label,
            driver_label,
            vram_size_label,
            link_speed_label,
            vulkan_info_frame,
        }
    }

    pub fn set_info(&self, gpu_info: &DeviceInfo) {
        let gpu_name = gpu_info
            .pci_info
            .as_ref()
            .and_then(|pci_info| {
                pci_info
                    .subsystem_pci_info
                    .model
                    .as_deref()
                    .or(pci_info.device_pci_info.model.as_deref())
            })
            .unwrap_or_default();
        self.gpu_name_label
            .set_markup(&format!("<b>{gpu_name}</b>",));

        let gpu_manufacturer = gpu_info
            .pci_info
            .as_ref()
            .and_then(|pci_info| {
                pci_info
                    .subsystem_pci_info
                    .vendor
                    .as_deref()
                    .or(pci_info.device_pci_info.model.as_deref())
            })
            .unwrap_or_default();
        self.gpu_manufacturer_label
            .set_markup(&format!("<b>{gpu_manufacturer}</b>",));

        let vbios_version = gpu_info.vbios_version.as_deref().unwrap_or("Unknown");
        self.vbios_version_label
            .set_markup(&format!("<b>{vbios_version}</b>",));

        self.driver_label
            .set_markup(&format!("<b>{}</b>", gpu_info.driver));

        let link_speed = gpu_info
            .link_info
            .current_speed
            .as_deref()
            .unwrap_or("Unknown");
        let link_width = gpu_info
            .link_info
            .current_width
            .as_deref()
            .unwrap_or("Unknown");
        self.link_speed_label
            .set_markup(&format!("<b>{link_speed} x{link_width}</b>",));

        if let Some(vulkan_info) = &gpu_info.vulkan_info {
            self.vulkan_info_frame.set_info(vulkan_info);
            self.vulkan_info_frame.container.show();
        } else {
            self.vulkan_info_frame.container.hide();
        }
    }

    pub fn set_stats(&self, stats: &DeviceStats) {
        let vram_size = stats.vram.total.map_or_else(
            || "Unknown".to_owned(),
            |size| (size / 1024 / 1024).to_string(),
        );
        self.vram_size_label
            .set_markup(&format!("<b>{vram_size} MiB</b>"));
    }
}

fn value_label() -> Label {
    Label::builder()
        .selectable(true)
        .halign(Align::Start)
        .build()
}
