//! Example: AuditPattern
//!
//! Demonstrates how to use AuditPattern to quickly assemble a complete
//! security audit report with findings, impact assessment, and remediation roadmap.

use renderreport::{Engine, patterns::AuditPattern};

fn main() -> renderreport::Result<()> {
    let engine = Engine::new()?;

    // Build audit using pattern
    let audit = AuditPattern::new()
        .with_hero_summary(serde_json::json!({
            "domain": "Web Application Security Audit",
            "score": 68,
            "grade": "C+",
            "computed_status": "bad",
            "verdict": "Critical security issues must be resolved before production deployment.",
            "metrics": [
                {"title": "Critical", "value": "5", "accent_color": "#e53e3e"},
                {"title": "High", "value": "12", "accent_color": "#dd6b20"},
                {"title": "Medium", "value": "23", "accent_color": "#ecc94b"},
            ],
            "top_actions": [
                "Patch XXE vulnerability in XML parser",
                "Enable HTTPS 1.3 minimum",
                "Implement request rate limiting"
            ],
            "positive_aspects": []
        }))
        .add_findings(vec![
            serde_json::json!({
                "type": "finding",
                "data": {
                    "title": "XML External Entity (XXE) Injection",
                    "description": "The /api/parse endpoint accepts untrusted XML without entity expansion protection.",
                    "severity": "critical",
                    "category": "CWE-611",
                    "affected": null,
                    "recommendation": "Disable XML entity expansion and validate against whitelist."
                }
            }),
            serde_json::json!({
                "type": "finding",
                "data": {
                    "title": "Missing HTTP Security Headers",
                    "description": "CSP, X-Frame-Options, and X-Content-Type-Options headers not configured.",
                    "severity": "critical",
                    "category": "HTTP Headers",
                    "affected": null,
                    "recommendation": "Add security headers to all HTTP responses via WAF."
                }
            }),
        ])
        .add_impact_grid(serde_json::json!({
            "title": "Impact Assessment",
            "user": {
                "label": "User Impact",
                "icon": "👤",
                "headline": "Data Exposure",
                "body": "User PII and session tokens at risk",
                "status": "bad"
            },
            "risk": {
                "label": "Exploitability",
                "icon": "⚠️",
                "headline": "High",
                "body": "Public exploits available",
                "status": "bad"
            },
            "conversion": {
                "label": "Compliance",
                "icon": "📋",
                "headline": "Non-Compliant",
                "body": "Violates OWASP Top 10",
                "status": "bad"
            }
        }))
        .add_roadmap(serde_json::json!({
            "columns": [
                {
                    "title": "Immediate (24h)",
                    "accent_color": "#e53e3e",
                    "items": [
                        {
                            "action": "Disable XML parsing or validate input",
                            "benefit": "Blocks XXE attacks",
                            "effort": "1h",
                            "priority": "critical",
                            "role": "Backend"
                        },
                        {
                            "action": "Enable security headers via WAF",
                            "benefit": "Mitigates XSS",
                            "effort": "30m",
                            "priority": "critical",
                            "role": "DevOps"
                        }
                    ]
                },
                {
                    "title": "Short-term (1 week)",
                    "accent_color": "#dd6b20",
                    "items": [
                        {
                            "action": "Upgrade TLS to 1.3 minimum",
                            "benefit": "Eliminates weak ciphers",
                            "effort": "4h",
                            "priority": "high",
                            "role": "DevOps"
                        },
                        {
                            "action": "Implement rate limiting",
                            "benefit": "Prevents brute force",
                            "effort": "2h",
                            "priority": "high",
                            "role": "Backend"
                        }
                    ]
                }
            ]
        }))
        .with_cta(serde_json::json!({
            "headline": "Schedule Remediation Review",
            "body": "Our team can assist with patch deployment and verification testing.",
            "action_label": "Book a Session",
            "action_url": "https://example.com/contact",
            "tone": "primary"
        }))
        .to_components();

    // Build report
    let report = engine
        .report("default")
        .title("Security Audit Report")
        .subtitle("Web Application Assessment")
        .metadata("author", "Security Team")
        .metadata("date", "April 2026")
        .metadata("footer_prefix", "Confidential");

    // Add components from pattern
    let mut report = report;
    for component in audit {
        report = report.add_raw_component(component);
    }

    let report = report.build();

    let pdf = engine.render_pdf(&report)?;
    std::fs::create_dir_all("examples/output")?;
    std::fs::write("examples/output/audit_example.pdf", &pdf)?;
    println!(
        "Audit report written to examples/output/audit_example.pdf ({} KB)",
        pdf.len() / 1024
    );

    Ok(())
}
