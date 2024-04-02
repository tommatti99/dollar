use financial_math_lib;

#[allow(dead_code)]
#[cfg(test)]
#[test]
fn test_i_aa_to_am() {
    assert_eq!(financial_math_lib::i_aa_to_am(0.0), 0 as f64);
    assert_eq!(financial_math_lib::i_aa_to_am(5.0), 0.40741237836483535);
    assert_eq!(financial_math_lib::i_aa_to_am(5.25), 0.4273127766158069);
}
#[allow(dead_code)]
#[cfg(test)]
#[test]
fn test_i_am_to_aa() {
    assert_eq!(financial_math_lib::i_am_to_aa(0.0), 0.0);
    assert_eq!(financial_math_lib::i_am_to_aa(1.0), 12.682503013196978);
    assert_eq!(financial_math_lib::i_am_to_aa(-2.5), -26.200165417334908);
    assert_eq!(financial_math_lib::i_am_to_aa(10.0), 213.8428376721003);
    assert_eq!(financial_math_lib::i_am_to_aa(0.5), 6.167781186449828);
}
#[allow(dead_code)]
#[cfg(test)]
#[test]
fn test_calcula_valor_futuro() {
    assert_eq!(financial_math_lib::calcula_valor_futuro(1000.0, 0.0, 5.0), 1000.0);
    assert_eq!(financial_math_lib::calcula_valor_futuro(1000.0, 5.0, 3.0), 1157.6250000000002);
    assert_eq!(financial_math_lib::calcula_valor_futuro(1000.0, -2.5, 4.0), 903.6878906249999);
    assert_eq!(financial_math_lib::calcula_valor_futuro(5000.0, 10.0, 8.0), 10717.944050000007);
    assert_eq!(financial_math_lib::calcula_valor_futuro(1200.0, 3.5, 6.5), 1500.6987462209113);
}
#[allow(dead_code)]
#[cfg(test)]
#[test]
fn test_calcula_valor_presente() {
    assert_eq!(financial_math_lib::calcula_valor_presente(1000.0, 0.0, 5.0), 1000.0);
    assert_eq!(financial_math_lib::calcula_valor_presente(1000.0, 5.0, 3.0), 863.837598531476);
    assert_eq!(financial_math_lib::calcula_valor_presente(1000.0, -2.5, 4.0), 1106.576740016279);
    assert_eq!(financial_math_lib::calcula_valor_presente(5000.0, 10.0, 8.0), 2332.536901048665);
    assert_eq!(financial_math_lib::calcula_valor_presente(1200.0, 3.5, 6.5), 959.5530106400341);
}
#[allow(dead_code)]
#[cfg(test)]
#[test]
fn test_calcula_vlr_futuro_serie_pgmt() {
    assert_eq!(financial_math_lib::calcula_vlr_futuro_serie_pgmt(200.0, 5.0, 1), 200.00000000000023);
    assert_eq!(financial_math_lib::calcula_vlr_futuro_serie_pgmt(200.0, 3.3, 5), 1068.2141741841974);
    assert_eq!(financial_math_lib::calcula_vlr_futuro_serie_pgmt(250.0, 4.8, 2), 512.0000000000005);
    assert_eq!(financial_math_lib::calcula_vlr_futuro_serie_pgmt(1500.0, 8.12, 10), 21854.0643315185);
    assert_eq!(financial_math_lib::calcula_vlr_futuro_serie_pgmt(180.0, 6.6, 3), 576.4240800000003);
    assert_eq!(financial_math_lib::calcula_vlr_futuro_serie_pgmt(1008.82, 0.4273, 6), 6117.949891906085);
}
#[allow(dead_code)]
#[cfg(test)]
#[test]
fn test_calcula_vlr_presente_serie_pgmt() {
    assert_eq!(financial_math_lib::calcula_vlr_presente_serie_pgmt(200.0, 5.0, 1), 190.47619047619068);
    assert_eq!(financial_math_lib::calcula_vlr_presente_serie_pgmt(200.0, 3.0, 5), 915.9414374389075);
    assert_eq!(financial_math_lib::calcula_vlr_presente_serie_pgmt(250.0, 4.2, 2), 470.17583931683157);
    assert_eq!(financial_math_lib::calcula_vlr_presente_serie_pgmt(1500.0, 8.9, 10),9669.004014893011);
    assert_eq!(financial_math_lib::calcula_vlr_presente_serie_pgmt(180.0, 6.1, 3), 480.25334197775675);
}
#[allow(dead_code)]
#[cfg(test)]
#[test]
fn test_acumula_taxa() {
    assert_eq!(financial_math_lib::acumula_taxa(0.0, 0.0), 0.0);
    assert_eq!(financial_math_lib::acumula_taxa(1.0, 5.0), 6.05);
    assert_eq!(financial_math_lib::acumula_taxa(-1.0, 5.8), 4.742000000000002);
    assert_eq!(financial_math_lib::acumula_taxa(80.3, -5.8), 69.84259999999998); 
}
#[allow(dead_code)]
#[cfg(test)]
#[test]
fn test_calcula_parcela_serie_pgmt_unif_usando_vlr_presente() {
    assert_eq!(financial_math_lib::calcula_parcela_serie_pgmt_unif_usando_vlr_presente(1000.0, 0.0, 5), 200.0);
    assert_eq!(financial_math_lib::calcula_parcela_serie_pgmt_unif_usando_vlr_presente(1000.0, 5.0, 3), 367.2085646312449);
    assert_eq!(financial_math_lib::calcula_parcela_serie_pgmt_unif_usando_vlr_presente(1000.0, -2.5, 4), 234.57275946122397);
    assert_eq!(financial_math_lib::calcula_parcela_serie_pgmt_unif_usando_vlr_presente(5000.0, 10.0, 8), 937.2200878740666);
    assert_eq!(financial_math_lib::calcula_parcela_serie_pgmt_unif_usando_vlr_presente(1200.0, 3.5, 6), 225.2018503983816);
}
#[allow(dead_code)]
#[cfg(test)]
#[test]
fn test_calcula_parcela_serie_pgmt_unif_usando_vlr_futuro() {
    assert_eq!(financial_math_lib::calcula_parcela_serie_pgmt_unif_usando_vlr_futuro(1000.0, 0.0, 5), 200.0);
    assert_eq!(financial_math_lib::calcula_parcela_serie_pgmt_unif_usando_vlr_futuro(1000.0, 5.0, 3), 317.2085646312449);
    assert_eq!(financial_math_lib::calcula_parcela_serie_pgmt_unif_usando_vlr_futuro(1000.0, -2.5, 4), 259.572759461224);
    assert_eq!(financial_math_lib::calcula_parcela_serie_pgmt_unif_usando_vlr_futuro(5000.0, 10.0, 8), 437.2200878740666);
    assert_eq!(financial_math_lib::calcula_parcela_serie_pgmt_unif_usando_vlr_futuro(1200.0, 3.5, 6), 183.2018503983816);
}
#[allow(dead_code)]
#[cfg(test)]
#[test]
fn test_calcula_variacao() {
    assert_eq!(financial_math_lib::calcula_variacao(0.0, 50.0), 100.0);
    assert_eq!(financial_math_lib::calcula_variacao(100.0, 0.0), -100.0);
    assert_eq!(financial_math_lib::calcula_variacao(75.0, 75.0), 0.0);
    assert_eq!(financial_math_lib::calcula_variacao(100.0, 120.0), 20.0);
    assert_eq!(financial_math_lib::calcula_variacao(80.0, 60.0), -25.0);  
}
#[allow(dead_code)]
#[cfg(test)]
#[test]
fn test_calcula_vpl() {
    let fluxos_de_caixa_1: Vec<f64> = vec![100.0, -50.0, 25.0, -12.5];
    assert_eq!(financial_math_lib::calcula_vpl(&fluxos_de_caixa_1, 0.0), 62.5);

    let fluxos_de_caixa_2: Vec<f64> = vec![100.0, -50.0, 25.0, -12.5];
    assert_eq!(financial_math_lib::calcula_vpl(&fluxos_de_caixa_2, 5.0), 61.19878034358113);

    let fluxos_de_caixa_3: Vec<f64> = vec![100.0, -50.0, 25.0, -12.5];
    assert_eq!(financial_math_lib::calcula_vpl(&fluxos_de_caixa_3, -0.1), 62.52497489978714);

    let fluxos_de_caixa_4: Vec<f64> = vec![1000.0, -500.0, 250.0, -125.0];
    assert_eq!(financial_math_lib::calcula_vpl(&fluxos_de_caixa_4, 0.02), 624.9499900079967);

    let fluxos_de_caixa_5: Vec<f64> = vec![100.0, -50.0, 25.0, -12.5];
    assert_eq!(financial_math_lib::calcula_vpl(&fluxos_de_caixa_5, 0.1), 62.474975099787876);
}
#[allow(dead_code)]
#[cfg(test)]
#[test]
fn test_calcula_tir() {
    let fluxos_de_caixa1: Vec<f64> = vec![-100.0, 200.0, 300.0];
    assert_eq!(financial_math_lib::calcula_tir(&fluxos_de_caixa1), 199.99999360471355);

    let fluxos_de_caixa2: Vec<f64> = vec![500.0, -200.0, -300.0, -400.0, -500.0];
    assert_eq!(financial_math_lib::calcula_tir(&fluxos_de_caixa2), 47.96268208777367);

    let fluxos_de_caixa3: Vec<f64> = vec![-1000.0, 200.0, -300.0, 400.0, -500.0];
    assert_eq!(financial_math_lib::calcula_tir(&fluxos_de_caixa3), 3191263.0864126612);

    let fluxos_de_caixa4: Vec<f64> = vec![-100000.0, 20000.0, 30000.0, 40000.0, 50000.0];
    assert_eq!(financial_math_lib::calcula_tir(&fluxos_de_caixa4), 12.82572689969918);

    let fluxos_de_caixa5: Vec<f64> = vec![-0.1, 0.02, 0.03, 0.04, 0.05];
    assert_eq!(financial_math_lib::calcula_tir(&fluxos_de_caixa5), 12.825258633856423);
}