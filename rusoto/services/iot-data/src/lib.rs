// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png"
)]
//! <p><fullname>AWS IoT</fullname> <p>AWS IoT-Data enables secure, bi-directional communication between Internet-connected things (such as sensors, actuators, embedded devices, or smart appliances) and the AWS cloud. It implements a broker for applications and things to publish messages over HTTP (Publish) and retrieve, update, and delete shadows. A shadow is a persistent representation of your things and their state in the AWS cloud.</p> <p>Find the endpoint address for actions in the AWS IoT data plane by running this CLI command:</p> <p> <code>aws iot describe-endpoint --endpoint-type iot:Data-ATS</code> </p> <p>The service name used by <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">AWS Signature Version 4</a> to sign requests is: <i>iotdevicegateway</i>.</p></p>
//!
//! If you're using the service, you're probably looking for [IotDataClient](struct.IotDataClient.html) and [IotData](trait.IotData.html).

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
