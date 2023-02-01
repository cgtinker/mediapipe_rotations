extern crate cgt_math;
use cgt_math::{Vector3, Quaternion};

pub fn main(face: &[[f32; 3]]) -> [Quaternion; 4]{
    let mut data = to_vectors(face);
    set_face_origin(&mut data);
    let mut rotation_data = [Quaternion::NAN; 4];
    face_rotation(&data, &mut rotation_data);
    mouth_corner_angles(&data, &mut rotation_data);
    chin_rotation(&data, &mut rotation_data);
    return rotation_data;
}

/// Converts data to Vector3s.
fn to_vectors(face: &[[f32; 3]]) -> [Vector3; 468] {
    let mut data: [Vector3; 468] = [Vector3::NAN; 468];
    for i in 0..468 {
        data[i] = Vector3::from_array(face[i]);
    }
    return data;
}

/// Sets face origin to approx center based on the canonial mesh geometry.
fn set_face_origin(data: &mut [Vector3; 468]) {
    let a = data[447].center(data[366]);
    let b = data[137].center(data[227]);
    let center = a.center(b);
    for i in 0..468 {
        data[i] -= center;
        let tmp = data[i];
        data[i].x = -tmp.x;
        data[i].y = tmp.z;
        data[i].z = -tmp.y;
    }
}

/// Approximation of the face rotation.
fn face_rotation(data: &[Vector3; 468],  rotation_data: &mut [Quaternion; 4]) {
    let normal = data[1].center(data[4]);
    let tangent = data[447].center(data[366]);
    let binormal = data[152];
    rotation_data[0] = Quaternion::from_rotation_axes(tangent, normal, binormal);
}

/// Approximate chin angle.
fn chin_rotation(data: &[Vector3; 468], rotation_data: &mut [Quaternion; 4]) {
    let mut nose_dir = data[2] - data[168];
    let mut chin_dir = data[200] - data[168];
    nose_dir.x = 0.0f32;
    chin_dir.x = 0.0f32;
    let angle = nose_dir.angle(chin_dir);
    rotation_data[1] = Quaternion::from_rotation_x((angle-0.314159)*1.15);
}

/// Approximate angle from mouth center to mouth corners.
fn mouth_corner_angles(data: &[Vector3; 468], rotation_data: &mut [Quaternion; 4])  {
    // center point of mouth corners gets projected on vector from upper to lower lip
    let corner_center = data[61].center(data[291]);
    // project vector AP onto vector AB, then add the resulting vector to point A.
    // A + dot(AP,AB) / dot(AB,AB) * AB
    let projected_center = corner_center + (corner_center - data[0]).project(data[17]-data[0]);
    // center point between upper and lower lip
    let mouth_height_center = data[0].center(data[17]);

    // vectors from center points to mouth corners
    let left_vec = data[61] - projected_center;
    let left_hv = data[61] - mouth_height_center;
    let right_vec =data[291] - projected_center;
    let right_hv = data[291] -  mouth_height_center;

    // angle between the vectors expecting users don't record upside down
    let mut right_corner_angle = left_vec.angle(left_hv);
    let mut left_corner_angle = right_vec.angle(right_hv);
    if mouth_height_center.z < projected_center.z {
        right_corner_angle *= -1.0;
        left_corner_angle *= -1.0;
    }

    rotation_data[2] = Quaternion::from_rotation_z(left_corner_angle);
    rotation_data[3] = Quaternion::from_rotation_z(right_corner_angle);
}

#[cfg(test)]
mod test {
    use cgt_math::{Vector3};
    #[test]
    fn impl_test() {
        let mut _face: [Vector3; 468] = [Vector3::NAN; 468];
        _face[0] = Vector3::from_array([0.5393765568733215, 0.5456897020339966, -0.04142116755247116]);
        _face[1] = Vector3::from_array([0.5391055941581726, 0.49038633704185486, -0.059236664324998856]);
        _face[2] = Vector3::from_array([0.5387719869613647, 0.5115706920623779, -0.03482384607195854]);
        _face[4] = Vector3::from_array([0.5388702154159546, 0.474435955286026, -0.061052966862916946]);
        _face[17] = Vector3::from_array([0.5392104387283325, 0.5884271860122681, -0.03995494544506073]);
        _face[61] = Vector3::from_array([0.49550861120224, 0.5762177109718323, -0.0135299451649189]);
        _face[137] = Vector3::from_array([0.4110361933708191, 0.5105783939361572, 0.07597390562295914]);
        _face[152] = Vector3::from_array([0.5383299589157104, 0.6849358677864075, -0.02242697775363922]);
        _face[168] = Vector3::from_array([0.5362387895584106, 0.4000281095504761, -0.006076775025576353]);
        _face[200] = Vector3::from_array([0.5387893915176392, 0.6273922324180603, -0.03420121967792511]);
        _face[227] = Vector3::from_array([0.40956437587738037, 0.47863948345184326, 0.07892382889986038]);
        _face[291] = Vector3::from_array([0.5816632509231567, 0.5723670125007629, -0.012652279809117317]);
        _face[366] = Vector3::from_array([0.6581948399543762, 0.5019081234931946, 0.0799756795167923]);
        _face[447] = Vector3::from_array([0.6591862440109253, 0.47007131576538086, 0.08288852870464325]);
        let _face = [[0.5393765568733215, 0.5456897020339966, -0.04142116755247116], [0.5391055941581726, 0.49038633704185486, -0.059236664324998856], [0.5387719869613647, 0.5115706920623779, -0.03482384607195854], [0.5282945036888123, 0.44496214389801025, -0.03778231516480446], [0.5388702154159546, 0.474435955286026, -0.061052966862916946], [0.5382651090621948, 0.45588919520378113, -0.05400751531124115], [0.5367512702941895, 0.41492027044296265, -0.01677008718252182], [0.4563271999359131, 0.42582187056541443, 0.025777511298656464], [0.536030650138855, 0.38307487964630127, -0.004146380815654993], [0.5360046625137329, 0.3636268675327301, -0.0031648168805986643], [0.5353055596351624, 0.3019507825374603, 0.0212551299482584], [0.5394298434257507, 0.5534129738807678, -0.04093744233250618], [0.5393937826156616, 0.5599790215492249, -0.0377962552011013], [0.5392295122146606, 0.5634527802467346, -0.0333540253341198], [0.5391577482223511, 0.5656709671020508, -0.03452669456601143], [0.539291262626648, 0.5712013840675354, -0.03716209530830383], [0.5393133163452148, 0.5790554285049438, -0.04036875441670418], [0.5392104387283325, 0.5884271860122681, -0.03995494544506073], [0.5388490557670593, 0.6079199314117432, -0.03089122474193573], [0.5389224886894226, 0.49982818961143494, -0.05455673113465309], [0.5292895436286926, 0.5015614032745361, -0.0402553454041481], [0.41652825474739075, 0.3851856291294098, 0.08841720968484879], [0.48786041140556335, 0.4358392655849457, 0.01307822298258543], [0.4769282042980194, 0.43811336159706116, 0.013629685156047344], [0.4665476083755493, 0.43819189071655273, 0.016710804775357246], [0.4526263475418091, 0.4325123131275177, 0.02655644714832306], [0.4971412122249603, 0.43085622787475586, 0.014937681145966053], [0.4739696979522705, 0.39987418055534363, 0.012962481938302517], [0.4851134717464447, 0.40019088983535767, 0.0135134132578969], [0.4633942246437073, 0.4027436077594757, 0.0160836074501276], [0.4560788869857788, 0.40858030319213867, 0.02023254707455635], [0.44408267736434937, 0.44345247745513916, 0.030758459120988846], [0.49509692192077637, 0.6410773992538452, -0.023230280727148056], [0.45274436473846436, 0.42283952236175537, 0.029859641566872597], [0.4113123118877411, 0.4475436508655548, 0.08107822388410568], [0.43154504895210266, 0.4357864260673523, 0.04362845793366432], [0.48312002420425415, 0.493509441614151, -0.007341782562434673], [0.5262979865074158, 0.5444602966308594, -0.03960059583187103], [0.5286420583724976, 0.5608121752738953, -0.0363708958029747], [0.5130049586296082, 0.5523579716682434, -0.034240707755088806], [0.5045117735862732, 0.5619449019432068, -0.027898017317056656], [0.5191164612770081, 0.5638891458511353, -0.032643213868141174], [0.511489748954773, 0.568135142326355, -0.02632623165845871], [0.489772766828537, 0.590095579624176, -0.01704622618854046], [0.5298275947570801, 0.4907253384590149, -0.058244384825229645], [0.5281183123588562, 0.47576186060905457, -0.05981796234846115], [0.4378986656665802, 0.39335304498672485, 0.027021290734410286], [0.5037962794303894, 0.4545607268810272, -0.002783752279356122], [0.5042212605476379, 0.49444448947906494, -0.031214872375130653], [0.5032143592834473, 0.4874681532382965, -0.02697909064590931], [0.4496409296989441, 0.4977332353591919, 0.00668328395113349], [0.5282511115074158, 0.45859140157699585, -0.05073504522442818], [0.46339452266693115, 0.37225937843322754, 0.009922550059854984], [0.44823992252349854, 0.3797854781150818, 0.01810934767127037], [0.4271314740180969, 0.3548755645751953, 0.07070238143205643], [0.5123859643936157, 0.38104841113090515, -0.0006028646021150053], [0.49526509642601013, 0.4039890766143799, 0.016292240470647812], [0.4814646244049072, 0.5775607228279114, -0.012068333104252815], [0.4198559820652008, 0.5926550626754761, 0.08580393344163895], [0.5127078890800476, 0.5024277567863464, -0.028063025325536728], [0.5220207571983337, 0.5064288377761841, -0.030304310843348503], [0.49550861120224, 0.5762177109718323, -0.0135299451649189], [0.5003643035888672, 0.5744331479072571, -0.016597844660282135], [0.4417397379875183, 0.37128159403800964, 0.02520066872239113], [0.5042440891265869, 0.5027806758880615, -0.026253828778862953], [0.4843499958515167, 0.3716394603252411, 0.00356048415414989], [0.48212066292762756, 0.35951000452041626, 0.0032978146336972713], [0.4717738628387451, 0.3131446838378906, 0.033247433602809906], [0.4345237612724304, 0.3622264862060547, 0.044761355966329575], [0.4779265522956848, 0.33466535806655884, 0.017786333337426186], [0.43115678429603577, 0.38917773962020874, 0.03796835243701935], [0.423907071352005, 0.3876688480377197, 0.061709508299827576], [0.5273179411888123, 0.5537184476852417, -0.03939032182097435], [0.5165296792984009, 0.5592389106750488, -0.03426670655608177], [0.5081693530082703, 0.5652257204055786, -0.02827833965420723], [0.5158169269561768, 0.5053805112838745, -0.026215558871626854], [0.4982127845287323, 0.5755364298820496, -0.01510543655604124], [0.5023660659790039, 0.5752633213996887, -0.023043224588036537], [0.5012860298156738, 0.573569655418396, -0.016987394541502], [0.5182729363441467, 0.49380674958229065, -0.04211307689547539], [0.5134624242782593, 0.5680109262466431, -0.02477157488465309], [0.5211969614028931, 0.5653509497642517, -0.028779609128832817], [0.5299447178840637, 0.5639599561691284, -0.03217487037181854], [0.5247274041175842, 0.6081019639968872, -0.03086233325302601], [0.5263291597366333, 0.5881555676460266, -0.0393654964864254], [0.5272090435028076, 0.5784914493560791, -0.039505209773778915], [0.5280940532684326, 0.5707616806030273, -0.035936079919338226], [0.52889084815979, 0.5660943984985352, -0.03337588533759117], [0.511528730392456, 0.5698468089103699, -0.025682728737592697], [0.5101426839828491, 0.5715786218643188, -0.0281795896589756], [0.5079005360603333, 0.5756210684776306, -0.030878715217113495], [0.5054973363876343, 0.5814633369445801, -0.029610123485326767], [0.49403679370880127, 0.5463863611221313, -0.02209751307964325], [0.4092373847961426, 0.5205044150352478, 0.11492794007062912], [0.5388068556785583, 0.5048977732658386, -0.04127857834100723], [0.5065829157829285, 0.5721282362937927, -0.020862219855189323], [0.5047453045845032, 0.5728315114974976, -0.0225095022469759], [0.5257020592689514, 0.5130876302719116, -0.03222890570759773], [0.5085645914077759, 0.5129925608634949, -0.01980021595954895], [0.5240153670310974, 0.5098298192024231, -0.03126627206802368], [0.49237504601478577, 0.4651830792427063, -0.0006815431988798082], [0.4747636318206787, 0.47693949937820435, 0.001154150697402656], [0.5016670823097229, 0.4956810474395752, -0.021783698350191116], [0.4444730579853058, 0.3302103877067566, 0.05054643750190735], [0.45146870613098145, 0.34370774030685425, 0.03051370196044445], [0.45908331871032715, 0.36099663376808167, 0.014120337553322315], [0.49854567646980286, 0.5976089835166931, -0.02347804419696331], [0.5086855888366699, 0.3614826500415802, -0.0021743150427937508], [0.5051345825195312, 0.3325268030166626, 0.010279136709868908], [0.5011478662490845, 0.304904580116272, 0.023082146421074867], [0.457605242729187, 0.4368656873703003, 0.021737439557909966], [0.432611346244812, 0.4558102488517761, 0.03529973328113556], [0.50304114818573, 0.425937682390213, 0.017088107764720917], [0.44340839982032776, 0.41435638070106506, 0.029822872951626778], [0.5114417672157288, 0.4431568384170532, -0.00543995900079608], [0.5107975602149963, 0.4859370291233063, -0.040979038923978806], [0.42088669538497925, 0.46911826729774475, 0.045429982244968414], [0.4404824674129486, 0.4640682339668274, 0.02358086407184601], [0.4549188017845154, 0.4668213725090027, 0.012643372640013695], [0.4757339060306549, 0.46205419301986694, 0.00815870612859726], [0.4910787343978882, 0.45414620637893677, 0.0067490204237401485], [0.5027368664741516, 0.44539177417755127, 0.004980885423719883], [0.5260746479034424, 0.41997209191322327, -0.013094126246869564], [0.42301031947135925, 0.49988624453544617, 0.03733913600444794], [0.4341281056404114, 0.41338151693344116, 0.03583474084734917], [0.5335381031036377, 0.4994158148765564, -0.05400587618350983], [0.5055608153343201, 0.46745213866233826, -0.009706413373351097], [0.4077022671699524, 0.4536905884742737, 0.11855167895555496], [0.5112234354019165, 0.43569788336753845, 0.004948691464960575], [0.5008437037467957, 0.4985467493534088, -0.01106911152601242], [0.449336439371109, 0.4246823489665985, 0.031832437962293625], [0.5100454688072205, 0.4777710735797882, -0.035637516528367996], [0.412071168422699, 0.5562041401863098, 0.10356728732585907], [0.5029941201210022, 0.42070168256759644, 0.020861392840743065], [0.5188063979148865, 0.46637070178985596, -0.045019179582595825], [0.44444435834884644, 0.6174544095993042, 0.021713361144065857], [0.4436899721622467, 0.6393517851829529, 0.03896640986204147], [0.4110361933708191, 0.5105783939361572, 0.07597390562295914], [0.4313267171382904, 0.599353015422821, 0.03882129117846489], [0.41754478216171265, 0.4158170521259308, 0.0749330222606659], [0.49120476841926575, 0.6605119705200195, -0.01932031661272049], [0.5347060561180115, 0.5044995546340942, -0.040611714124679565], [0.49609261751174927, 0.480499267578125, -0.007526136003434658], [0.42215612530708313, 0.4412575960159302, 0.054340872913599014], [0.4669623076915741, 0.42918822169303894, 0.01868022419512272], [0.47645077109336853, 0.4297221899032593, 0.015945622697472572], [0.49953871965408325, 0.5789080858230591, -0.02156156487762928], [0.42523884773254395, 0.5302857160568237, 0.03532646223902702], [0.5125440955162048, 0.6835384368896484, -0.019061807543039322], [0.47576695680618286, 0.6676148176193237, 0.005655041895806789], [0.46051180362701416, 0.6563569903373718, 0.01999945193529129], [0.5357351899147034, 0.3319934606552124, 0.008231406100094318], [0.5383299589157104, 0.6849358677864075, -0.02242697775363922], [0.4856719374656677, 0.4280902147293091, 0.015799202024936676], [0.49427276849746704, 0.4249054491519928, 0.01766219176352024], [0.4999402165412903, 0.42275354266166687, 0.02051945962011814], [0.4262598156929016, 0.4132561683654785, 0.04786045476794243], [0.49336254596710205, 0.41239362955093384, 0.017486222088336945], [0.48393410444259644, 0.41007018089294434, 0.01589919999241829], [0.4748070240020752, 0.40985581278800964, 0.016261255368590355], [0.4654201865196228, 0.41265326738357544, 0.018668264150619507], [0.45941466093063354, 0.41679251194000244, 0.021793052554130554], [0.4104306101799011, 0.4161781072616577, 0.10673171281814575], [0.46058619022369385, 0.4275175929069519, 0.02245231904089451], [0.5389326214790344, 0.5224853754043579, -0.03425224497914314], [0.5038509368896484, 0.5351134538650513, -0.025389529764652252], [0.5123379230499268, 0.4990238547325134, -0.03127945959568024], [0.5243335366249084, 0.5254087448120117, -0.03389807045459747], [0.5362387895584106, 0.4000281095504761, -0.006076775025576353], [0.4592329263687134, 0.6350471377372742, 0.007639880292117596], [0.4742354452610016, 0.6484009623527527, -0.0055008032359182835], [0.5126402974128723, 0.6680847406387329, -0.030981453135609627], [0.43099769949913025, 0.6201233863830566, 0.06296034157276154], [0.500067949295044, 0.4167976379394531, 0.01960335299372673], [0.5193182826042175, 0.4426356256008148, -0.020477227866649628], [0.5385500192642212, 0.6695515513420105, -0.033771052956581116], [0.4927144944667816, 0.6769975423812866, -0.009151455946266651], [0.41438382863998413, 0.5436504483222961, 0.06863824278116226], [0.5195410251617432, 0.5673073530197144, -0.02974807843565941], [0.5179680585861206, 0.5707485675811768, -0.03272799775004387], [0.5163869261741638, 0.5766875743865967, -0.035648249089717865], [0.514722466468811, 0.5854763984680176, -0.035254839807748795], [0.5101463198661804, 0.6038274168968201, -0.028783611953258514], [0.5049793124198914, 0.572160005569458, -0.020958926528692245], [0.5017416477203369, 0.5713386535644531, -0.020991714671254158], [0.4982958137989044, 0.5696004629135132, -0.020606230944395065], [0.48591604828834534, 0.5611273050308228, -0.016862686723470688], [0.44099077582359314, 0.5330650806427002, 0.012345374561846256], [0.5189084410667419, 0.4304949939250946, -0.00834271777421236], [0.5122590661048889, 0.4082258939743042, 0.014509220607578754], [0.5052052736282349, 0.4123963415622711, 0.01849115826189518], [0.5070728063583374, 0.5704489946365356, -0.020022490993142128], [0.43835777044296265, 0.5727842450141907, 0.022068677470088005], [0.5220285654067993, 0.40440118312835693, 0.0008473344496451318], [0.5022370219230652, 0.6225767731666565, -0.025113124400377274], [0.537762463092804, 0.44166985154151917, -0.041128382086753845], [0.5269597768783569, 0.43288102746009827, -0.02624514140188694], [0.5372235178947449, 0.4285353422164917, -0.02845321036875248], [0.5125043988227844, 0.46662089228630066, -0.023814251646399498], [0.5386776924133301, 0.6497411131858826, -0.03741976618766785], [0.5387893915176392, 0.6273922324180603, -0.03420121967792511], [0.5200775265693665, 0.6275151371955872, -0.032216668128967285], [0.47632667422294617, 0.5992678999900818, -0.011331466026604176], [0.4919435381889343, 0.5123246908187866, -0.010532490909099579], [0.48791009187698364, 0.6135408282279968, -0.018429633229970932], [0.46589481830596924, 0.513129711151123, -0.006127085071057081], [0.481586754322052, 0.5285582542419434, -0.012046818621456623], [0.45625588297843933, 0.5405631065368652, -0.0018627019599080086], [0.5152919292449951, 0.6487531065940857, -0.03422924131155014], [0.506203293800354, 0.47737541794776917, -0.017703156918287277], [0.46465691924095154, 0.6141577363014221, -0.0025238848756998777], [0.47950267791748047, 0.6291934847831726, -0.01218437124043703], [0.4678187072277069, 0.5812645554542542, -0.006722338031977415], [0.4276546537876129, 0.556722104549408, 0.03475899249315262], [0.4522341787815094, 0.5866605639457703, 0.004309881012886763], [0.41994708776474, 0.5735791325569153, 0.05810987204313278], [0.4728196859359741, 0.5514569282531738, -0.010404436849057674], [0.5126041769981384, 0.4547555446624756, -0.014654182828962803], [0.5146175026893616, 0.4915596842765808, -0.043898168951272964], [0.507698118686676, 0.49860745668411255, -0.033078473061323166], [0.518881618976593, 0.47979646921157837, -0.05075252801179886], [0.5031973123550415, 0.39661717414855957, 0.012502440251410007], [0.4853345453739166, 0.38977837562561035, 0.011240248568356037], [0.47014838457107544, 0.3891586661338806, 0.01167051587253809], [0.45735761523246765, 0.3927956223487854, 0.015520278364419937], [0.4482956528663635, 0.4009431004524231, 0.021975869312882423], [0.4415496587753296, 0.429842084646225, 0.03593844920396805], [0.40956437587738037, 0.47863948345184326, 0.07892382889986038], [0.4501686692237854, 0.44848018884658813, 0.024268748238682747], [0.4613718092441559, 0.45077764987945557, 0.0171333160251379], [0.4757692217826843, 0.44921624660491943, 0.012509027495980263], [0.4896542727947235, 0.4444221556186676, 0.010917294770479202], [0.5005593299865723, 0.4379560053348541, 0.011688658967614174], [0.5080955028533936, 0.43165791034698486, 0.011580782011151314], [0.4083842635154724, 0.4874557852745056, 0.12078772485256195], [0.5083500742912292, 0.5038292407989502, -0.02814079448580742], [0.5198215246200562, 0.45395663380622864, -0.0311280507594347], [0.5222501754760742, 0.4897908866405487, -0.05257744714617729], [0.5285133123397827, 0.4981161057949066, -0.04937983304262161], [0.5225433111190796, 0.4932933747768402, -0.047613900154829025], [0.5118697285652161, 0.5090029239654541, -0.02519918419420719], [0.5303324460983276, 0.49889183044433594, -0.05247364193201065], [0.5314108729362488, 0.5037569999694824, -0.04037381336092949], [0.5071867108345032, 0.42079102993011475, 0.019163886085152626], [0.5140441656112671, 0.4226510524749756, 0.011895724572241306], [0.5177216529846191, 0.4231056571006775, 0.0037556374445557594], [0.45575326681137085, 0.4199496805667877, 0.025413399562239647], [0.4501262903213501, 0.4152737259864807, 0.02613266557455063], [0.5470937490463257, 0.44433966279029846, -0.037507738918066025], [0.6145680546760559, 0.42047637701034546, 0.027935590595006943], [0.5481180548667908, 0.5009241104125977, -0.04002391919493675], [0.6507744789123535, 0.3778284788131714, 0.09219939261674881], [0.5840055346488953, 0.4328056871891022, 0.014469954185187817], [0.595051109790802, 0.43424785137176514, 0.015299048274755478], [0.6052600741386414, 0.4335508346557617, 0.0186906848102808], [0.6183124780654907, 0.42694705724716187, 0.028847163543105125], [0.5745596885681152, 0.4284532964229584, 0.016006266698241234], [0.5973992943763733, 0.39612865447998047, 0.014504830352962017], [0.5861213207244873, 0.3969690203666687, 0.014578119851648808], [0.607885479927063, 0.3983069658279419, 0.01804952509701252], [0.6150435209274292, 0.4036523997783661, 0.02222394198179245], [0.6269676685333252, 0.4373959004878998, 0.03326363489031792], [0.58126300573349, 0.6376608610153198, -0.022192908450961113], [0.6178250908851624, 0.41734012961387634, 0.032122887670993805], [0.656991183757782, 0.43912771344184875, 0.0849565863609314], [0.6385780572891235, 0.42883092164993286, 0.046500179916620255], [0.591498076915741, 0.4895845651626587, -0.006191495805978775], [0.5522400140762329, 0.5432901382446289, -0.03941244259476662], [0.5498926043510437, 0.5600013136863708, -0.03611600399017334], [0.5651447176933289, 0.5500308871269226, -0.033799897879362106], [0.5733200311660767, 0.5589044690132141, -0.027210373431444168], [0.5590667128562927, 0.5622467398643494, -0.0322854146361351], [0.5663710236549377, 0.565822184085846, -0.025858774781227112], [0.5870350003242493, 0.5860812664031982, -0.01605287194252014], [0.5481212139129639, 0.4900924563407898, -0.058023516088724136], [0.5492367148399353, 0.47499117255210876, -0.05961956828832626], [0.6325402855873108, 0.3873898684978485, 0.02962684817612171], [0.5697224736213684, 0.4523876905441284, -0.0019612370524555445], [0.5721186995506287, 0.4918055236339569, -0.030445434153079987], [0.5725561380386353, 0.48482978343963623, -0.026191217824816704], [0.6238266229629517, 0.4916151165962219, 0.00888512097299099], [0.5482667684555054, 0.4578742980957031, -0.050392549484968185], [0.6076899170875549, 0.36820748448371887, 0.011444850824773312], [0.6225255727767944, 0.374696284532547, 0.0201601292937994], [0.6407487392425537, 0.3481728732585907, 0.07384856790304184], [0.5595424175262451, 0.3796747028827667, -0.00010850797116290778], [0.5760228037834167, 0.40148115158081055, 0.017304224893450737], [0.5950859785079956, 0.5728045105934143, -0.01092925202101469], [0.6490576267242432, 0.583953857421875, 0.08939369022846222], [0.5640167593955994, 0.5003725290298462, -0.027486950159072876], [0.5549418330192566, 0.5051008462905884, -0.02999703399837017], [0.5816632509231567, 0.5723670125007629, -0.012652279809117317], [0.5768464207649231, 0.5709107518196106, -0.015810132026672363], [0.6285287737846375, 0.36591964960098267, 0.02745886892080307], [0.5720207691192627, 0.5001171827316284, -0.02551022171974182], [0.5870963335037231, 0.36872702836990356, 0.004573070909827948], [0.5891050696372986, 0.3563934564590454, 0.004460759926587343], [0.5977137684822083, 0.3090844750404358, 0.0350351482629776], [0.6346151828765869, 0.35603633522987366, 0.047551460564136505], [0.592610776424408, 0.3312179148197174, 0.019344031810760498], [0.6386743783950806, 0.3828691244125366, 0.04079413414001465], [0.6447741985321045, 0.38089093565940857, 0.06497053802013397], [0.5513066649436951, 0.5527771711349487, -0.039206016808748245], [0.5618264675140381, 0.557255208492279, -0.033851832151412964], [0.5697606801986694, 0.5624350905418396, -0.027688397094607353], [0.5609428286552429, 0.5034842491149902, -0.025729281827807426], [0.5790080428123474, 0.571843683719635, -0.01436723954975605], [0.5750337839126587, 0.5720064640045166, -0.022311709821224213], [0.5758712291717529, 0.5700861811637878, -0.016164904460310936], [0.5589064955711365, 0.4924286901950836, -0.041606876999139786], [0.5641998052597046, 0.5657093524932861, -0.024327950552105904], [0.556747317314148, 0.563886284828186, -0.028477279469370842], [0.5482593774795532, 0.5632098913192749, -0.032112229615449905], [0.552787184715271, 0.606981098651886, -0.030689392238855362], [0.551860511302948, 0.587221622467041, -0.039218395948410034], [0.5511987209320068, 0.5775328278541565, -0.03932859003543854], [0.5502901077270508, 0.5699037909507751, -0.03575197607278824], [0.5492327213287354, 0.5654089450836182, -0.03306904062628746], [0.5658643841743469, 0.5674201846122742, -0.025071175768971443], [0.5674381256103516, 0.5690145492553711, -0.02764792926609516], [0.5697149634361267, 0.5730509161949158, -0.030279362574219704], [0.5720012784004211, 0.5786460041999817, -0.02898099273443222], [0.5827734470367432, 0.5426657795906067, -0.021208489313721657], [0.6579755544662476, 0.5116228461265564, 0.11910383403301239], [0.570684015750885, 0.5691874027252197, -0.02009335160255432], [0.5725046396255493, 0.5698258876800537, -0.021796369925141335], [0.551584005355835, 0.5120236277580261, -0.03191971033811569], [0.5677906274795532, 0.5104033946990967, -0.019266022369265556], [0.5531330108642578, 0.5087076425552368, -0.031076855957508087], [0.5812199115753174, 0.4621998071670532, 0.00044891255674883723], [0.5989096760749817, 0.47261953353881836, 0.0027250826824456453], [0.574016809463501, 0.4928531050682068, -0.021057702600955963], [0.6241486668586731, 0.3245784640312195, 0.05315640568733215], [0.6184062957763672, 0.3387952744960785, 0.03254959359765053], [0.6117125749588013, 0.356611043214798, 0.015770206227898598], [0.5783832669258118, 0.5942956209182739, -0.022658664733171463], [0.5631185173988342, 0.3599269390106201, -0.0016040507471188903], [0.565945029258728, 0.3306255340576172, 0.011075913906097412], [0.5690615773200989, 0.3026425838470459, 0.023997588083148003], [0.6136886477470398, 0.4315723180770874, 0.023776203393936157], [0.6383844017982483, 0.44889500737190247, 0.03828459233045578], [0.5685449838638306, 0.42402321100234985, 0.01806408166885376], [0.6271677613258362, 0.4085882902145386, 0.03232429549098015], [0.5619791746139526, 0.4414730370044708, -0.0048042843118309975], [0.5657901167869568, 0.484055757522583, -0.04038003459572792], [0.6496739387512207, 0.461353063583374, 0.048811737447977066], [0.6314799785614014, 0.457562655210495, 0.026278678327798843], [0.6177796125411987, 0.46127092838287354, 0.0148153742775321], [0.5972622632980347, 0.4579317569732666, 0.009799331426620483], [0.5818246006965637, 0.4512162208557129, 0.007900704629719257], [0.5700579881668091, 0.44323551654815674, 0.005801311228424311], [0.5473132133483887, 0.41931501030921936, -0.012794763781130314], [0.6483827233314514, 0.49184656143188477, 0.04065072163939476], [0.6360906958580017, 0.40692687034606934, 0.038770273327827454], [0.5442387461662292, 0.49909770488739014, -0.05378779023885727], [0.568791389465332, 0.4652644693851471, -0.009017097763717175], [0.6587693691253662, 0.44517552852630615, 0.12274843454360962], [0.5613992810249329, 0.4341946244239807, 0.005500857252627611], [0.5746243596076965, 0.4956279993057251, -0.01033792831003666], [0.6213201284408569, 0.4190686047077179, 0.034213364124298096], [0.5658141374588013, 0.4757334589958191, -0.034950148314237595], [0.6558049917221069, 0.5473236441612244, 0.10753224790096283], [0.568376362323761, 0.4187382459640503, 0.021852798759937286], [0.5572679042816162, 0.46488896012306213, -0.044499628245830536], [0.6288056373596191, 0.6102635264396667, 0.02400578372180462], [0.6284052729606628, 0.6319267749786377, 0.04143876954913139], [0.6581948399543762, 0.5019081234931946, 0.0799756795167923], [0.6406877040863037, 0.5914586782455444, 0.04165635257959366], [0.6508512496948242, 0.4082297682762146, 0.07857080549001694], [0.584840714931488, 0.6565584540367126, -0.01830422133207321], [0.5428935885429382, 0.5041785836219788, -0.04053707793354988], [0.5783315896987915, 0.47755569219589233, -0.006667683366686106], [0.6475350260734558, 0.4337221682071686, 0.057733405381441116], [0.6045804619789124, 0.42458438873291016, 0.020676275715231895], [0.5952441096305847, 0.42587023973464966, 0.01760530099272728], [0.5777402520179749, 0.575501024723053, -0.020763812586665154], [0.6465796232223511, 0.5224421620368958, 0.03849995508790016], [0.5637105107307434, 0.6814456582069397, -0.01855369657278061], [0.5988718867301941, 0.662399411201477, 0.0071624284610152245], [0.613196849822998, 0.6499045491218567, 0.022056683897972107], [0.5858503580093384, 0.42497217655181885, 0.017230957746505737], [0.5771358013153076, 0.42234426736831665, 0.018999578431248665], [0.5714517831802368, 0.42060860991477966, 0.021633194759488106], [0.6433987617492676, 0.40628594160079956, 0.05088871717453003], [0.5775860548019409, 0.4096487760543823, 0.018665095791220665], [0.5867933034896851, 0.40664392709732056, 0.017100565135478973], [0.595945417881012, 0.4057128131389618, 0.01786966063082218], [0.605162501335144, 0.407900333404541, 0.020435823127627373], [0.6112875938415527, 0.41160911321640015, 0.023872731253504753], [0.6563431620597839, 0.40833359956741333, 0.11067742854356766], [0.6106488704681396, 0.42250269651412964, 0.0244623851031065], [0.5730975866317749, 0.5321528911590576, -0.0247099120169878], [0.564509391784668, 0.49704888463020325, -0.030748799443244934], [0.5531939268112183, 0.5241274237632751, -0.03358938544988632], [0.6152068376541138, 0.628734827041626, 0.00939431693404913], [0.6008973121643066, 0.6430656313896179, -0.004220918286591768], [0.5641950964927673, 0.6661003232002258, -0.030520929023623466], [0.6393856406211853, 0.6119562983512878, 0.06602669507265091], [0.5710926055908203, 0.41469722986221313, 0.020603608340024948], [0.5550001859664917, 0.4414367079734802, -0.020095935091376305], [0.5828749537467957, 0.6731531620025635, -0.008208928629755974], [0.6554301977157593, 0.5349749326705933, 0.07228405028581619], [0.5584043860435486, 0.5657663345336914, -0.029441846534609795], [0.559944748878479, 0.5690032839775085, -0.032212644815444946], [0.561566948890686, 0.5748662352561951, -0.035223573446273804], [0.5631653666496277, 0.5835557579994202, -0.03479744866490364], [0.5670565366744995, 0.6015403270721436, -0.028324328362941742], [0.5725444555282593, 0.5690162777900696, -0.020284155383706093], [0.5758458971977234, 0.5679912567138672, -0.020272905007004738], [0.579128623008728, 0.5659650564193726, -0.019861489534378052], [0.5907142758369446, 0.5567800998687744, -0.015899252146482468], [0.6322658061981201, 0.5261864066123962, 0.014641917310655117], [0.5544535517692566, 0.4293649196624756, -0.007873793132603168], [0.559441328048706, 0.40697526931762695, 0.01500456128269434], [0.5661892890930176, 0.4105184078216553, 0.019260156899690628], [0.5704407095909119, 0.5675538182258606, -0.01950867846608162], [0.6347966194152832, 0.565520703792572, 0.02449108473956585], [0.550226628780365, 0.4035809338092804, 0.0010589944431558251], [0.5744330286979675, 0.6195653080940247, -0.02444196678698063], [0.5474759936332703, 0.4321669340133667, -0.025971902534365654], [0.5624914169311523, 0.46485471725463867, -0.023323919624090195], [0.5571794509887695, 0.6260343194007874, -0.03184923529624939], [0.5995875597000122, 0.5944427847862244, -0.010190781205892563], [0.5835764408111572, 0.5087422132492065, -0.009684305638074875], [0.5882741808891296, 0.6096051335334778, -0.017575044184923172], [0.6087172031402588, 0.5080347061157227, -0.004772037733346224], [0.5940924286842346, 0.5243125557899475, -0.011028662323951721], [0.618417501449585, 0.5346361994743347, -0.0002843265829142183], [0.5618180632591248, 0.6470291614532471, -0.03379053995013237], [0.5688233971595764, 0.47521868348121643, -0.017046943306922913], [0.6105613112449646, 0.6084120273590088, -0.0010197829687967896], [0.5962517261505127, 0.6244761943817139, -0.010992264375090599], [0.6077817678451538, 0.5757640600204468, -0.0054702446796], [0.6443629264831543, 0.5488772988319397, 0.03758399561047554], [0.622142493724823, 0.5803001523017883, 0.006167474668473005], [0.6506220102310181, 0.5650826692581177, 0.06150037422776222], [0.6029602289199829, 0.5464298725128174, -0.009244618937373161], [0.5616832971572876, 0.4531089663505554, -0.013999704271554947], [0.5624372363090515, 0.48988455533981323, -0.043243587017059326], [0.5690189599990845, 0.49625831842422485, -0.03241632506251335], [0.5581611394882202, 0.47839605808258057, -0.05017412081360817], [0.5683250427246094, 0.39476102590560913, 0.013218112289905548], [0.5861682295799255, 0.38676732778549194, 0.012334712781012058], [0.6013524532318115, 0.3852633833885193, 0.013322008773684502], [0.6138926148414612, 0.3881692588329315, 0.0176046472042799], [0.6226580739021301, 0.39564967155456543, 0.02428956888616085], [0.6289211511611938, 0.4236328601837158, 0.03855883702635765], [0.6591862440109253, 0.47007131576538086, 0.08288852870464325], [0.6214540004730225, 0.4426519572734833, 0.026432892307639122], [0.6107332706451416, 0.44572553038597107, 0.019082527607679367], [0.596592128276825, 0.4452442228794098, 0.0140089625492692], [0.5826090574264526, 0.44143643975257874, 0.012159180827438831], [0.5716407895088196, 0.43577098846435547, 0.012499258853495121], [0.5640374422073364, 0.43003392219543457, 0.012311349622905254], [0.6582388877868652, 0.4787902235984802, 0.12497151643037796], [0.5681978464126587, 0.501468300819397, -0.027570683509111404], [0.5553380250930786, 0.4526737928390503, -0.0307219997048378], [0.5553054213523865, 0.48862072825431824, -0.05225840210914612], [0.5489792227745056, 0.4974365234375, -0.04915696382522583], [0.5549169182777405, 0.4922398030757904, -0.04724806174635887], [0.5647149085998535, 0.5068439841270447, -0.024707859382033348], [0.5474226474761963, 0.4983176589012146, -0.05229652673006058], [0.5460473895072937, 0.5032849907875061, -0.04020218923687935], [0.5643815398216248, 0.4191043972969055, 0.01990233175456524], [0.5580031871795654, 0.42142391204833984, 0.012491739355027676], [0.5547962784767151, 0.4220934510231018, 0.0042008752934634686], [0.614969789981842, 0.414588987827301, 0.027553515508770943], [0.6206554770469666, 0.4098378121852875, 0.028316890820860863]];
    }
}
