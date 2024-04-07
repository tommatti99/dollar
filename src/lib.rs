//! Biblioteca abriga funções para cálculos de matemática financeira
///
/// 
///
/// Transforma taxa anual inserida em taxa mensal
/// 
/// # Arguments
///  * i_ao_ano - taxa ao ano em formato 10.0 para 10%
/// 
pub fn i_aa_to_am(i_ao_ano: f64) -> f64 {
    let i_ao_mes: f64 = ((1.0 + (i_ao_ano / 100.0)).powf(1.0 / 12.0) - 1.0) * 100.0;
    return i_ao_mes;
}
/// Transforma taxa mensal inserida em taxa anual
/// 
/// # Arguments
///  * i_ao_mes - taxa ao ano em formato 10.0 para 10%
///
pub fn i_am_to_aa(i_ao_mes: f64) -> f64 {
    let i_ao_ano: f64 = (((1.0 + (i_ao_mes / 100.0)).powf(12.0)) - 1.0 ) * 100.0;
    return i_ao_ano;
}
/// Capitalizacao para um valor presente unico unico
/// 
/// # Arguments
///  * valor_presente - formato 1000.00 para R$1000,00
///  * i - taxa/juros, formato 10.0 para 10% 
///  * n - numero de periodos, formato 12.0 para 12 periodos 
/// 
pub fn calcula_valor_futuro(valor_presente: f64, i: f64, n: f64) -> f64 {
    let i = i / 100.0;
    let valor_futuro:f64 = valor_presente * (1.0 + i).powf(n);
    return valor_futuro;
}
/// Retorno para valor presente a partir de valor futuro unico unico
/// 
/// # Arguments
///  * valor_presente - formato 1000.00 para R$1000,00
///  * i - taxa/juros, formato 10.0 para 10% 
///  * n - numero de periodos, formato 12.0 para 12 periodos
/// 
pub fn calcula_valor_presente(valor_futuro: f64, i: f64, n: f64) -> f64 {
    let i: f64 = i / 100.0;
    let valor_presente:f64 = valor_futuro / ((1.0 + i).powf(n));
    return valor_presente;
}
/// Calcula o valor futuro em uma série de pagamentos de parcela fixa
/// 
/// # Arguments
///  * parcela - formato 1000.00 para R$1000,00
///  * i - taxa/juros, formato 10.0 para 10% 
///  * n - numero de periodos, formato 12.0 para 12 periodos
/// 
pub fn calcula_vlr_futuro_serie_pgmt(parcela: f64, i: f64, n: u64) -> f64 {
    let i = i / 100.0;
    let vlr_futuro_serie_parcelas: f64 = (parcela * ( 1.0 - (1.0 + i).powf(-(n as f64))) / i) * (1.0 + i).powf(n as f64);
    return vlr_futuro_serie_parcelas;
}
/// Calcula o valor presente em uma série de pagamentos de parcela fixa
/// 
/// # Arguments
///  * parcela - formato 1000.00 para R$1000,00
///  * i - taxa/juros, formato 10.0 para 10% 
///  * n - numero de periodos, formato 12.0 para 12 periodos
/// 
pub fn calcula_vlr_presente_serie_pgmt(parcela: f64, i: f64, n: u64) -> f64 {
    let i = i / 100.0;
    let vlr_presente_serie_parcelas: f64 = parcela * ( 1.0 - (1.0 + i).powf(-(n as f64))) / i;
    return vlr_presente_serie_parcelas;
}
/// Acumula duas taxas 
/// 
/// # Arguments
///  * tx_anterior - formato 10.0 para 10% ​
///  * tx_atual - formato 10.0 para 10% 
/// 
pub fn acumula_taxa(tx_anterior: f64, tx_atual: f64) -> f64 {
    let tx_anterior = tx_anterior / 100.0;
    let tx_atual = tx_atual / 100.0;

    let tx_acumulada:f64 = (((1.0 + tx_anterior) * (1.0 + tx_atual)) - 1.0) * 100.0;
    return tx_acumulada;
}
///
/// Acumula as taxas passadas no argumento de forma vetorizada/historico
/// 
/// # Arguments
///  * vetor_taxa - deve ser informado em formato '`Vec<f64>`', sendo os elementos em formato ( 3% -> 3.00, 20,5% -> 20.50)
/// 
pub fn acumula_taxa_vetorizado(vetor_taxa: Vec<f64>) -> Vec<f64> {
    
    let mut vetor_taxa_acumulado: Vec<f64> = Vec::new();

    for (i, _) in vetor_taxa.iter().enumerate() {
        if i - 1 > 0{
            vetor_taxa_acumulado.push(acumula_taxa(vetor_taxa[i - 1], vetor_taxa[i]));
        } else {
            vetor_taxa_acumulado.push(vetor_taxa[i]);
        }
    }
    
    return vetor_taxa_acumulado;
}
///  Calcula o valor da parcela (R) de uma série de pagamentos a partir do valor presente da série
/// 
/// # Arguments
///  * vlr_presente_serie_parcelas - formato 1000.00 para R$1000,00
///  * i - taxa/juros, formato 10.0 para 10% 
///  * n - numero de periodos, formato 12.0 para 12 periodos
/// 
pub fn calcula_parcela_serie_pgmt_unif_usando_vlr_presente(vlr_presente_serie_parcelas: f64, i: f64, n: u64) -> f64 {
    if i == 0.0 {
        return vlr_presente_serie_parcelas / (n as f64)
    } else {
        let i: f64 = i / 100.0;
        let parcela: f64 = (vlr_presente_serie_parcelas * i) / (1.0 - ( 1.0 + i).powf(-(n as f64)));
        return parcela;
    }    
}
///  Calcula o valor da parcela (R) de uma série de pagamentos a partir do valor futuro da série
/// 
/// # Arguments
///  * vlr_futuro_serie_parcelas - formato 1000.00 para R$1000,00
///  * i - taxa/juros, formato 10.0 para 10% 
///  * n - numero de periodos, formato 12.0 para 12 periodos
/// 
pub fn calcula_parcela_serie_pgmt_unif_usando_vlr_futuro(vlr_futuro_serie_parcelas: f64, i: f64, n: u64) -> f64 {

    
    if i == 0.0 {
        return vlr_futuro_serie_parcelas / (n as f64)
    } else {
        let i: f64 = i / 100.0;
        let vlr_presente_serie_parcelas = vlr_futuro_serie_parcelas / (1.0 + i).powf(n as f64);
        let parcela: f64 = (vlr_presente_serie_parcelas * i) / (1.0 - ( 1.0 + i).powf(-(n as f64)));
        return parcela;
    }    
}
/// Calcula a variação percentual entre um valor inicial e um final
/// 
/// # Arguments
///  * vlr_inicial - formato 100.00 para 100,00
///  * vlr_final - formato 100.00 para 100,00
/// 
pub fn calcula_variacao(vlr_inicial: f64, vlr_final: f64) -> f64 {
    if vlr_inicial == vlr_final {
        return 0.0; 
    } else if vlr_inicial == 0.0 {
        return 100.0;
    } else if vlr_final == 0.0 {
        return -100.0;
    } else {
    let variacao: f64 = ( vlr_final - vlr_inicial ) / vlr_inicial;
        return variacao * 100.0;
    }
}
/// Calcula o valor presente liquido VPL para certo fluxo de caixa 
/// 
/// # Arguments
///  * fluxos_de_caixa - Vec => (Fc1, Fc2, ..., Fcn) sendo Fcn no formato 1000.00 para R$1000,00
///  * i - taxa/juros, formato 10.0 para 10% 
/// 
/// # Atenção!!
/// O numero de períodos utilizados é o mesmo que o número de fluxos inseridos no vetor, caso exista algum periodo sem movimentação, deverá ser inserido o valor 0.00 no lugar
/// 
pub fn calcula_vpl(fluxos_de_caixa: &Vec<f64>, i: f64) -> f64 {
    let mut n: f64 = 1.00;
    let mut vpl: f64 = 0.00; 

    for fc in fluxos_de_caixa {
        let fcp: f64 = calcula_valor_presente(fc.clone(), i.clone(), n);
        n += 1.0;
        vpl += fcp;
    }
    return vpl;
}
/// Apenas a derivada função de fluxo de caixa igualado a zero (VPL) 
fn calcula_derivada_fluxo_de_caixa(fluxos_de_caixa: &Vec<f64>, i: f64) -> f64 {
    let mut acumulador: f64 = 0.0;
    let i = i / 100.0;
    let mut n = 1.0;


    for fc in fluxos_de_caixa {
        if n == 1.0 {
            acumulador += -fc * (n as f64) * ( 1.0 + i ).powf(-(n as f64) - (n as f64));
            n += 1.0;
        } else {
            acumulador += fc * (-(n as f64) - (n as f64) ) * ( 1.0 + i).powf(-(n as f64)- (n as f64));
            n += 1.0;
        }
    }
    return acumulador;
}
/// 
/// Calcula a TIR para o fluxo de caixa inserido
/// 
/// # Description
/// A função busca a raíz utilizando o método de Newton Raphson
/// 
/// # Arguments
///  * fluxos_de_caixa - Vec => (Fc1, Fc2, ..., Fcn) sendo Fcn no formato 1000.00 para R$1000,00
/// 
pub fn calcula_tir(fluxos_de_caixa: &Vec<f64>) -> f64 {
    const MAX_ITERACOES: u64 = 100_000_000_000_000_000;
    const PRECISAO: f64 = 0.000_001;

        let mut taxa_chute: f64 = 0.0;
        let mut iteracao: u64 = 0;

        while iteracao < MAX_ITERACOES {
            let vpl: f64 = calcula_vpl(fluxos_de_caixa,taxa_chute);
            let resultado_derivada_na_taxa_chute: f64 = calcula_derivada_fluxo_de_caixa(fluxos_de_caixa,taxa_chute);

            if resultado_derivada_na_taxa_chute.abs() < PRECISAO {
                break;
            } else if vpl.abs() < PRECISAO {
                break;
            } else {
                taxa_chute = taxa_chute - (vpl / resultado_derivada_na_taxa_chute);
            }

            iteracao += 1;
        }
    return taxa_chute;
}

#[cfg(test)]
mod test_financial_math_lib {
    use super::*;
    
    #[test]
    fn test_i_aa_to_am() {
        assert_eq!(i_aa_to_am(0.0), 0 as f64);
        assert_eq!(i_aa_to_am(5.0), 0.40741237836483535);
        assert_eq!(i_aa_to_am(5.25), 0.4273127766158069);
    }

    #[test]
    fn test_i_am_to_aa() {
        assert_eq!(i_am_to_aa(0.0), 0.0);
        assert_eq!(i_am_to_aa(1.0), 12.682503013196978);
        assert_eq!(i_am_to_aa(-2.5), -26.200165417334908);
        assert_eq!(i_am_to_aa(10.0), 213.8428376721003);
        assert_eq!(i_am_to_aa(0.5), 6.167781186449828);
    }

    #[test]
    fn test_calcula_valor_futuro() {
        assert_eq!(calcula_valor_futuro(1000.0, 0.0, 5.0), 1000.0);
        assert_eq!(calcula_valor_futuro(1000.0, 5.0, 3.0), 1157.6250000000002);
        assert_eq!(calcula_valor_futuro(1000.0, -2.5, 4.0), 903.6878906249999);
        assert_eq!(calcula_valor_futuro(5000.0, 10.0, 8.0), 10717.944050000007);
        assert_eq!(calcula_valor_futuro(1200.0, 3.5, 6.5), 1500.6987462209113);
    }

    #[test]
    fn test_calcula_valor_presente() {
        assert_eq!(calcula_valor_presente(1000.0, 0.0, 5.0), 1000.0);
        assert_eq!(calcula_valor_presente(1000.0, 5.0, 3.0), 863.837598531476);
        assert_eq!(calcula_valor_presente(1000.0, -2.5, 4.0), 1106.576740016279);
        assert_eq!(calcula_valor_presente(5000.0, 10.0, 8.0), 2332.536901048665);
        assert_eq!(calcula_valor_presente(1200.0, 3.5, 6.5), 959.5530106400341);
    }

    #[test]
    fn test_calcula_vlr_futuro_serie_pgmt() {
        assert_eq!(calcula_vlr_futuro_serie_pgmt(200.0, 5.0, 1), 200.00000000000023);
        assert_eq!(calcula_vlr_futuro_serie_pgmt(200.0, 3.3, 5), 1068.2141741841974);
        assert_eq!(calcula_vlr_futuro_serie_pgmt(250.0, 4.8, 2), 512.0000000000005);
        assert_eq!(calcula_vlr_futuro_serie_pgmt(1500.0, 8.12, 10), 21854.0643315185);
        assert_eq!(calcula_vlr_futuro_serie_pgmt(180.0, 6.6, 3), 576.4240800000003);
        assert_eq!(calcula_vlr_futuro_serie_pgmt(1008.82, 0.4273, 6), 6117.949891906085);
    }

    #[test]
    fn test_calcula_vlr_presente_serie_pgmt() {
        assert_eq!(calcula_vlr_presente_serie_pgmt(200.0, 5.0, 1), 190.47619047619068);
        assert_eq!(calcula_vlr_presente_serie_pgmt(200.0, 3.0, 5), 915.9414374389075);
        assert_eq!(calcula_vlr_presente_serie_pgmt(250.0, 4.2, 2), 470.17583931683157);
        assert_eq!(calcula_vlr_presente_serie_pgmt(1500.0, 8.9, 10),9669.004014893011);
        assert_eq!(calcula_vlr_presente_serie_pgmt(180.0, 6.1, 3), 480.25334197775675);
    }

    #[test]
    fn test_acumula_taxa() {
        assert_eq!(acumula_taxa(0.0, 0.0), 0.0);
        assert_eq!(acumula_taxa(1.0, 5.0), 6.05);
        assert_eq!(acumula_taxa(-1.0, 5.8), 4.742000000000002);
        assert_eq!(acumula_taxa(80.3, -5.8), 69.84259999999998); 
    }

    #[test]
    fn test_calcula_parcela_serie_pgmt_unif_usando_vlr_presente() {
        assert_eq!(calcula_parcela_serie_pgmt_unif_usando_vlr_presente(1000.0, 0.0, 5), 200.0);
        assert_eq!(calcula_parcela_serie_pgmt_unif_usando_vlr_presente(1000.0, 5.0, 3), 367.2085646312449);
        assert_eq!(calcula_parcela_serie_pgmt_unif_usando_vlr_presente(1000.0, -2.5, 4), 234.57275946122397);
        assert_eq!(calcula_parcela_serie_pgmt_unif_usando_vlr_presente(5000.0, 10.0, 8), 937.2200878740666);
        assert_eq!(calcula_parcela_serie_pgmt_unif_usando_vlr_presente(1200.0, 3.5, 6), 225.2018503983816);
    }

    #[test]
    fn test_calcula_parcela_serie_pgmt_unif_usando_vlr_futuro() {
        assert_eq!(calcula_parcela_serie_pgmt_unif_usando_vlr_futuro(1000.0, 0.0, 5), 200.0);
        assert_eq!(calcula_parcela_serie_pgmt_unif_usando_vlr_futuro(1000.0, 5.0, 3), 317.2085646312449);
        assert_eq!(calcula_parcela_serie_pgmt_unif_usando_vlr_futuro(1000.0, -2.5, 4), 259.572759461224);
        assert_eq!(calcula_parcela_serie_pgmt_unif_usando_vlr_futuro(5000.0, 10.0, 8), 437.2200878740666);
        assert_eq!(calcula_parcela_serie_pgmt_unif_usando_vlr_futuro(1200.0, 3.5, 6), 183.2018503983816);
    }

    #[test]
    fn test_calcula_variacao() {
        assert_eq!(calcula_variacao(0.0, 50.0), 100.0);
        assert_eq!(calcula_variacao(100.0, 0.0), -100.0);
        assert_eq!(calcula_variacao(75.0, 75.0), 0.0);
        assert_eq!(calcula_variacao(100.0, 120.0), 20.0);
        assert_eq!(calcula_variacao(80.0, 60.0), -25.0);  
    }

    #[test]
    fn test_calcula_vpl() {
        let fluxos_de_caixa_1: Vec<f64> = vec![100.0, -50.0, 25.0, -12.5];
        assert_eq!(calcula_vpl(&fluxos_de_caixa_1, 0.0), 62.5);
    
        let fluxos_de_caixa_2: Vec<f64> = vec![100.0, -50.0, 25.0, -12.5];
        assert_eq!(calcula_vpl(&fluxos_de_caixa_2, 5.0), 61.19878034358113);
    
        let fluxos_de_caixa_3: Vec<f64> = vec![100.0, -50.0, 25.0, -12.5];
        assert_eq!(calcula_vpl(&fluxos_de_caixa_3, -0.1), 62.52497489978714);
    
        let fluxos_de_caixa_4: Vec<f64> = vec![1000.0, -500.0, 250.0, -125.0];
        assert_eq!(calcula_vpl(&fluxos_de_caixa_4, 0.02), 624.9499900079967);
    
        let fluxos_de_caixa_5: Vec<f64> = vec![100.0, -50.0, 25.0, -12.5];
        assert_eq!(calcula_vpl(&fluxos_de_caixa_5, 0.1), 62.474975099787876);
    }

    #[test]
    fn test_calcula_tir() {
        let fluxos_de_caixa1: Vec<f64> = vec![-100.0, 200.0, 300.0];
        assert_eq!(calcula_tir(&fluxos_de_caixa1), 199.99999360471355);
    
        let fluxos_de_caixa2: Vec<f64> = vec![500.0, -200.0, -300.0, -400.0, -500.0];
        assert_eq!(calcula_tir(&fluxos_de_caixa2), 47.96268208777367);
    
        let fluxos_de_caixa3: Vec<f64> = vec![-1000.0, 200.0, -300.0, 400.0, -500.0];
        assert_eq!(calcula_tir(&fluxos_de_caixa3), 3191263.0864126612);
    
        let fluxos_de_caixa4: Vec<f64> = vec![-100000.0, 20000.0, 30000.0, 40000.0, 50000.0];
        assert_eq!(calcula_tir(&fluxos_de_caixa4), 12.82572689969918);
    
        let fluxos_de_caixa5: Vec<f64> = vec![-0.1, 0.02, 0.03, 0.04, 0.05];
        assert_eq!(calcula_tir(&fluxos_de_caixa5), 12.825258633856423);
    }
}