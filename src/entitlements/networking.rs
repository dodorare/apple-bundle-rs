use serde::{Deserialize, Serialize};

/// Networking
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Networking {
    /// The APIs an app can use to customize networking features.
    ///
    /// To add this entitlement to an iOS app or a Mac App Store app, enable the Network
    /// Extensions capability in Xcode.
    ///
    /// To add this entitlement to a macOS app distributed outside of the Mac App Store,
    /// perform the following steps: 1. In the Certificates, Identifiers and Profiles
    /// section of the developer site, enable the Network Extension capability for
    /// your Developer ID–signed app. Generate a new provisioning profile and download it.
    /// 2. On your Mac, drag the downloaded provisioning profile to Xcode to install it.
    /// 3. In your Xcode project, enable manual signing and select the provisioning
    /// profile downloaded earlier and its associated certificate. 4. Update the
    /// project’s entitlements.plist to include the
    /// com.apple.developer.networking.networkextension key and the values of the
    /// entitlement.
    ///
    /// ## Availability
    /// * iOS 9.0+
    /// * macOS 10.11+
    ///
    /// ## Framework
    /// * Network Extension
    #[serde(
        rename = "com.apple.developer.networking.networkextension",
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_vec_enum_option"
    )]
    pub network_extensions: Option<Vec<NetworkExtensions>>,
    /// The API an app can use to create and control a custom system VPN configuration.
    ///
    /// With the Personal VPN Entitlement enabled, your app can use the NEVPNManager class
    /// to manage a Personal VPN configuration.
    ///
    /// To add this entitlement to your app, enable the Personal VPN capability in Xcode.
    /// When the entitlement is enabled, Xcode sets the value to allow-vpn.
    ///
    /// ## Availability
    /// * iOS 8.0+
    /// * macOS 10.10+
    ///
    /// ## Framework
    /// * Network Extension
    #[serde(
        rename = "com.apple.developer.networking.vpn.api",
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_vec_enum_option"
    )]
    pub personal_vpn: Option<Vec<PersonalVPN>>,
    /// The associated domains for specific services, such as shared web credentials,
    /// universal links, and App Clips.
    ///
    /// This key specifies a list of domains for each service enabled. Add an associated
    /// domain to the list in the following format:
    /// ```swift
    /// <service>:<fully qualified domain>
    /// ```
    ///
    /// ### Services include:
    /// * webcredentials
    ///
    /// Use this service for shared web credentials.
    /// * applinks
    ///
    /// Use this service for universal links.
    /// * activitycontinuation
    ///
    /// Use this service for Handoff.
    /// * appclips
    ///
    /// Use this service for an App Clip.
    ///
    /// ### Note
    /// In macOS 11 and later and iOS 14 and later, apps request
    /// apple-app-site-association files from an Apple-managed content delivery
    /// network (CDN) dedicated to associated domains, instead of directly from your
    /// web server. If the CDN has an old version of the file, or doesn’t already have
    /// a copy of the file, it connects to your web server to obtain the latest
    /// version.
    ///
    /// While developing your app, if you use a private web server that’s unreachable from
    /// the public internet, you can use the alternate mode feature to bypass the CDN
    /// and connect directly to your private domain. Add a query string to your
    /// associated domains entitlement as follows:
    ///
    /// ```swift
    /// <service>:<fully qualified domain>?mode=<alternate mode>
    /// ```
    ///
    /// Where alternate mode is one of the following:
    /// * developer
    ///
    /// Specifies that only devices in developer mode can access the domain. In this mode,
    /// you can use any valid SSL certificate on your web server, including a
    /// certificate the system doesn’t trust. Make sure you don’t expose your users to
    /// security issues, such as man-in-the-middle attacks. As an added precaution,
    /// only apps signed with a development profile can use developer mode, and users
    /// must opt in on any device they use.
    /// * managed
    ///
    /// Specifies that only devices managed with a mobile device management (MDM) profile
    /// can access the domain. This mode requires consent from the MDM administrator.
    /// * developer+managed
    ///
    /// Specifies that only devices that are in both developer and managed modes at the
    /// same time can access the domain.
    ///
    /// To add this entitlement to your app, enable the Associated Domains capability in
    /// Xcode.
    ///
    /// ## Availability
    /// * iOS 9.0+
    /// * macOS 10.15+
    /// * tvOS 9.0+
    /// * watchOS 6.0+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.developer.associated-domains",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub associated_domains: Option<Vec<String>>,
    /// A Boolean value that indicates whether an app can send or receive IP multicast
    /// traffic.
    ///
    /// Your app must have this entitlement to send or receive IP multicast or broadcast
    /// on iOS. It also allows your app to browse and advertise arbitrary Bonjour
    /// service types.
    ///
    /// This entitlement requires permission from Apple before you can use it in your app.
    /// Request permission from the Multicast Networking Entitlement Request page.
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * macOS 11.0+
    /// * tvOS 14.0+
    ///
    /// ## Framework
    /// * Network
    #[serde(
        rename = "com.apple.developer.networking.multicast",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub networking_multicast: Option<bool>,
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.developer.associated-domains.applinks.read-write",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub associated_domains_applinks_read_write: Option<bool>,
}

/// Network Extensions
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum NetworkExtensions {
    /// The APIs you use to proxy DNS queries.
    #[serde(rename = "dns-proxy")]
    DnsProxy,
    /// The APIs you use to proxy TCP and UDP connections.
    #[serde(rename = "app-proxy-provider")]
    AppProxyProvider,
    /// The filter APIs you use to allow or deny network connections created by other apps
    /// on the system.
    #[serde(rename = "content-filter-provider")]
    ContentFilterProvider,
    /// The APIs you use to tunnel IP packets to a remote network using any custom
    /// tunneling protocol.
    #[serde(rename = "packet-tunnel-provider")]
    PacketTunnelProvider,
    /// The APIs you use to proxy DNS queries, when signed with a Developer ID profile.
    #[serde(rename = "dns-proxy-systemextension")]
    DnsProxySystemextension,
    /// The APIs you use to proxy TCP and UDP connections, when signed with a Developer ID
    /// profile.
    #[serde(rename = "app-proxy-provider-systemextension")]
    AppProxyProviderSystemextension,
    /// The filter APIs you use to allow or deny network connections created by other apps
    /// on the system, when signed with a Developer ID profile.
    #[serde(rename = "content-filter-provider-systemextension")]
    ContentFilterProviderSystemExtensions,
    /// The APIs you use to tunnel IP packets to a remote network using any custom
    /// tunneling protocol, when signed with a Developer ID profile.
    #[serde(rename = "packet-tunnel-provider-systemextension")]
    PacketTunnelProviderSystemExtension,
    /// The APIs you use to create and manage a system-wide DNS configuration.
    #[serde(rename = "dns-settings")]
    DnsSettings,
    /// The APIs you use for providing functionality similar to Apple Push Notification
    /// Service when access to the wider internet is unavailable.
    #[serde(rename = "app-push-provider")]
    AppPushProvider,
}

/// Personal VPN
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PersonalVPN {
    #[serde(rename = "allow-vpn")]
    AllowVpn,
}
