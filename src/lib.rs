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